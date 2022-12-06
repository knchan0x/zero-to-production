-- Create Subscription Tokens Table
CREATE TABLE `subscription_tokens`(
    `subscription_token` TEXT NOT NULL,
    PRIMARY KEY (`subscription_token`(102)),
    `subscriber_id` VARCHAR(36) NOT NULL,
    FOREIGN KEY (`subscriber_id`)
    REFERENCES `subscriptions` (`id`)
);