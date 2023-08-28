-- Add down migration script here
DELETE FROM News
WHERE competition_id == 1;
DELETE FROM Competitions
WHERE id == 1;
DELETE FROM Users
WHERE id == 1;
