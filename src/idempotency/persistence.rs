use super::IdempotencyKey;
use actix_web::body::to_bytes;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::MySqlPool;
use sqlx::{MySql, Transaction};

#[derive(Debug, Serialize, Deserialize)]
struct HeaderPairRecord {
    name: String,
    value: Vec<u8>,
}

pub async fn get_saved_response(
    pool: &MySqlPool,
    idempotency_key: &IdempotencyKey,
    user_id: uuid::fmt::Hyphenated,
) -> Result<Option<HttpResponse>, anyhow::Error> {
    let saved_response = sqlx::query!(
        r#"
            SELECT 
                `response_status_code` as `response_status_code!`, 
                `response_headers` as `response_headers!`,
                `response_body` as `response_body!`
            FROM `idempotency`
            WHERE 
                `user_id` = ? AND
                `idempotency_key` = ?
        "#,
        user_id,
        idempotency_key.as_ref()
    )
    .fetch_optional(pool)
    .await?;
    if let Some(r) = saved_response {
        let status_code = StatusCode::from_u16(r.response_status_code.try_into()?)?;
        let mut response = HttpResponse::build(status_code);
        for header in r.response_headers.as_array().unwrap() {
            let unwrap_header: HeaderPairRecord = serde_json::from_value(header.to_owned())?;
            response.append_header((unwrap_header.name, unwrap_header.value));
        }
        Ok(Some(response.body(r.response_body)))
    } else {
        Ok(None)
    }
}

pub async fn save_response(
    mut transaction: Transaction<'static, MySql>,
    idempotency_key: &IdempotencyKey,
    user_id: uuid::fmt::Hyphenated,
    http_response: HttpResponse,
) -> Result<HttpResponse, anyhow::Error> {
    let (response_head, body) = http_response.into_parts();
    let body = to_bytes(body).await.map_err(|e| anyhow::anyhow!("{}", e))?;
    let status_code = response_head.status().as_u16() as i16;
    let headers = {
        let mut h = Vec::with_capacity(response_head.headers().len());
        for (name, value) in response_head.headers().iter() {
            let name = name.as_str().to_owned();
            let value = value.as_bytes().to_owned();
            h.push(HeaderPairRecord { name, value });
        }
        h
    };
    sqlx::query_unchecked!(
        r#"
            UPDATE `idempotency`
            SET 
                `response_status_code` = ?, 
                `response_headers` = ?,
                `response_body` = ?
            WHERE
                `user_id` = ? AND
                `idempotency_key` = ?
        "#,
        status_code,
        json!(headers),
        body.as_ref(),
        user_id,
        idempotency_key.as_ref()
    )
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;

    let http_response = response_head.set_body(body).map_into_boxed_body();
    Ok(http_response)
}

#[allow(clippy::large_enum_variant)]
pub enum NextAction {
    // Return transaction for later usage
    StartProcessing(Transaction<'static, MySql>),
    ReturnSavedResponse(HttpResponse),
}

pub async fn try_processing(
    pool: &MySqlPool,
    idempotency_key: &IdempotencyKey,
    user_id: uuid::fmt::Hyphenated,
) -> Result<NextAction, anyhow::Error> {
    let mut transaction = pool.begin().await?;
    let n_inserted_rows = sqlx::query!(
        r#"
            INSERT IGNORE INTO `idempotency` (
                `user_id`, 
                `idempotency_key`,
                `created_at`
            ) 
            VALUES (?, ?, ?) 
        "#,
        user_id,
        idempotency_key.as_ref(),
        Utc::now()
    )
    .execute(&mut transaction)
    .await?
    .rows_affected();
    if n_inserted_rows > 0 {
        Ok(NextAction::StartProcessing(transaction))
    } else {
        let saved_response = get_saved_response(pool, idempotency_key, user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("We expected a saved response, we didn't find it"))?;
        Ok(NextAction::ReturnSavedResponse(saved_response))
    }
}
