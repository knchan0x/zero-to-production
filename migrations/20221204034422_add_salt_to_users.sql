-- Add Salt
ALTER TABLE `users` 
ADD COLUMN `salt` TEXT NOT NULL;