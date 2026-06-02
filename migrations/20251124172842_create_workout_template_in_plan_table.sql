-- Add migration script here

CREATE TABLE "workout_template_in_plan" (
  "workout_plan_id" uuid NOT NULL,
  "workout_template_id" uuid NOT NULL,
  PRIMARY KEY ("workout_plan_id", "workout_template_id"),

  FOREIGN KEY ("workout_plan_id") REFERENCES "workout_plan" ("id")
    ON UPDATE CASCADE
    ON DELETE CASCADE,
  FOREIGN KEY ("workout_template_id") REFERENCES "workout_template" ("id")
    ON UPDATE CASCADE
    ON DELETE CASCADE
);
