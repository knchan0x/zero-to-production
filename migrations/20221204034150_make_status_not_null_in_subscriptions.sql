-- Make Status not NULL in Subscriptions
START TRANSACTION;
    UPDATE `subscriptions` 
    SET `status` = 'confirmed' 
    WHERE `status` IS NULL;

    ALTER TABLE `subscriptions` 
    MODIFY COLUMN `status` TEXT NOT NULL;
COMMIT;