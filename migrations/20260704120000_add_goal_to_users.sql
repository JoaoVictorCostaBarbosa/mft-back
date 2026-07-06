-- Add migration script here

CREATE TYPE goal_enum AS ENUM ('muscle', 'loss', 'strength', 'health');

ALTER TABLE "users" ADD COLUMN "goal" goal_enum;
