-- Add migration script here

CREATE TABLE "workout_plan_routine_item" (
  "id" uuid PRIMARY KEY,
  "workout_plan_id" uuid NOT NULL,
  "workout_template_id" uuid,
  "item_type" routine_item_type_enum NOT NULL,
  "day_of_week" day_of_week_enum,
  "position" int,

  CONSTRAINT workout_plan_routine_item_schedule_check CHECK (
    ("day_of_week" IS NOT NULL AND "position" IS NULL)
    OR ("day_of_week" IS NULL AND "position" IS NOT NULL)
  ),
  CONSTRAINT workout_plan_routine_item_type_check CHECK (
    ("item_type" = 'workout' AND "workout_template_id" IS NOT NULL)
    OR ("item_type" = 'rest' AND "workout_template_id" IS NULL)
  ),
  CONSTRAINT workout_plan_routine_item_position_check CHECK (
    "position" IS NULL OR "position" > 0
  ),

  FOREIGN KEY ("workout_plan_id") REFERENCES "workout_plan" ("id")
    ON UPDATE CASCADE
    ON DELETE CASCADE,
  FOREIGN KEY ("workout_template_id") REFERENCES "workout_template" ("id")
    ON UPDATE CASCADE
    ON DELETE CASCADE
);

CREATE UNIQUE INDEX workout_plan_routine_item_plan_day_idx
  ON "workout_plan_routine_item" ("workout_plan_id", "day_of_week")
  WHERE "day_of_week" IS NOT NULL;

CREATE UNIQUE INDEX workout_plan_routine_item_plan_position_idx
  ON "workout_plan_routine_item" ("workout_plan_id", "position")
  WHERE "position" IS NOT NULL;
