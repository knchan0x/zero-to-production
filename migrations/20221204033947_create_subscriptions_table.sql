-- Create Subscriptions Table
CREATE TABLE `subscriptions`(
   `id` VARCHAR(36) NOT NULL,
   PRIMARY KEY (`id`),
   `email` TEXT NOT NULL,
   UNIQUE KEY `email` (`email`(102)),
   `name` TEXT NOT NULL,
   `subscribed_at` DATETIME(3) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;