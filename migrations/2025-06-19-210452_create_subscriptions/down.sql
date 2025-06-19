-- This file should undo anything in `up.sql`
DROP TRIGGER set_updated_at ON subscriptions;
DROP TABLE subscriptions;
