-- Add migration script here

CREATE TABLE "workout_log" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid NOT NULL,
  "workout_plan_id" uuid NOT NULL,
  "workout_template_id" uuid NOT NULL,
  "started_at" timestamptz DEFAULT current_timestamp NOT NULL,
  "finished_at" timestamptz,
  "status" workout_session_status_enum NOT NULL DEFAULT 'in_progress',
  "deleted_at" timestamptz,

  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON UPDATE CASCADE,
  FOREIGN KEY ("workout_plan_id") REFERENCES "workout_plan" ("id") ON UPDATE CASCADE,
  FOREIGN KEY ("workout_template_id") REFERENCES "workout_template" ("id") ON UPDATE CASCADE
);

CREATE UNIQUE INDEX workout_log_user_in_progress_idx
  ON "workout_log" ("user_id")
  WHERE "status" = 'in_progress' AND "deleted_at" IS NULL;
