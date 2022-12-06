-- Create Issue Delivery Queue
CREATE TABLE `issue_delivery_queue` (
   `newsletter_issue_id` VARCHAR(36) NOT NULL
   REFERENCES `newsletter_issues` (`newsletter_issue_id`),
   `subscriber_email` TEXT NOT NULL,
   PRIMARY KEY(`newsletter_issue_id`, `subscriber_email`(102))
);