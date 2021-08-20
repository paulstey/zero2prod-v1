-- Add migration script here
-- migrations/{timestamp}_create_subscriptions_table.sql
-- Create Subscriptions Table
DROP TABLE IF EXISTS subscriptions;

CREATE TABLE subscriptions (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL
);