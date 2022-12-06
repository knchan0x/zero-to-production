-- Create Idempotency Table
CREATE TABLE `idempotency` (
   `user_id` VARCHAR(36) NOT NULL
   REFERENCES `user` (`id`),
   `idempotency_key` VARCHAR(36) NOT NULL,
   `response_status_code` SMALLINT NOT NULL,
   `response_headers` JSON NOT NULL,
   `response_body` LONGBLOB NOT NULL,
   `created_at` DATETIME(3) NOT NULL,
   PRIMARY KEY(`user_id`, `idempotency_key`)
);