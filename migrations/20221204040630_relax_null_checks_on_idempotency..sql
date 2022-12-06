-- Relax null checks
ALTER TABLE `idempotency` 
MODIFY COLUMN `response_status_code` SMALLINT;

ALTER TABLE `idempotency`
MODIFY COLUMN `response_headers` JSON;

ALTER TABLE `idempotency`
MODIFY COLUMN `response_body` LONGBLOB;