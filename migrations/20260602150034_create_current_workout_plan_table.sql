-- Add migration script here

CREATE TABLE "current_workout_plan" (
  "user_id" uuid NOT NULL PRIMARY KEY,
  "workout_plan_id" uuid NOT NULL,

  FOREIGN KEY ("workout_plan_id") REFERENCES "workout_plan" ("id")
    ON UPDATE CASCADE
    ON DELETE CASCADE,
  FOREIGN KEY ("user_id") REFERENCES "users" ("id")
    ON UPDATE CASCADE
    ON DELETE CASCADE
);
