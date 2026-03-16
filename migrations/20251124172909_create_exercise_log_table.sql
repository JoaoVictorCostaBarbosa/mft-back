-- Add migration script here

CREATE TABLE "exercise_log" (
  "id" uuid PRIMARY KEY,
  "workout_log_id" uuid NOT NULL,
  "exercise_id" uuid NOT NULL,

  FOREIGN KEY ("workout_log_id") REFERENCES "workout_log" ("id") ON UPDATE CASCADE,
  FOREIGN KEY ("exercise_id") REFERENCES "exercise" ("id") ON UPDATE CASCADE
);
