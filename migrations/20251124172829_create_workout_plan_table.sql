-- Add migration script here

CREATE TABLE "workout_plan" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid NOT NULL,
  "name" varchar(100) NOT NULL,
  "created_at" timestamptz DEFAULT current_timestamp NOT NULL,
  "updated_at" timestamptz,
  "deleted_at" timestamptz,

  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON UPDATE CASCADE
);
