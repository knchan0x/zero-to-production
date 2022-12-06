-- Create User Table
CREATE TABLE `users`(
    `user_id` VARCHAR(36) NOT NULL,
    PRIMARY KEY (`user_id`),
    `username` TEXT NOT NULL,
    UNIQUE KEY `username` (`username`(102)),
    `password` TEXT NOT NULL
);