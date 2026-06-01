-- Add migration script here

CREATE TABLE "workout_exercise" (
  "workout_id" uuid NOT NULL,
  "exercise_id" uuid NOT NULL,
  PRIMARY KEY ("workout_id", "exercise_id"),

  FOREIGN KEY ("workout_id") REFERENCES "workout_template" ("id")
    ON UPDATE CASCADE
    ON DELETE CASCADE,
  FOREIGN KEY ("exercise_id") REFERENCES "exercise" ("id")
    ON UPDATE CASCADE
    ON DELETE CASCADE
);
