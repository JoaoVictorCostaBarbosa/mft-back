-- Add migration script here

CREATE TABLE "workout_log" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid NOT NULL,
  "name" varchar(100) NOT NULL,
  "workout_id" uuid,
  "started_at" timestamptz DEFAULT current_timestamp NOT NULL,
  "finished_at" timestamptz,
  "deleted_at" timestamptz,

  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON UPDATE CASCADE,
  FOREIGN KEY ("workout_id") REFERENCES "workout_template" ("id") ON UPDATE CASCADE
);
