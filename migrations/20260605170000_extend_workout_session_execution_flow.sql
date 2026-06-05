ALTER TYPE workout_session_status_enum ADD VALUE IF NOT EXISTS 'cancelled';

ALTER TABLE "exercise_log"
  ADD COLUMN IF NOT EXISTS "position" int NOT NULL DEFAULT 1;

ALTER TABLE "set_log"
  ADD COLUMN IF NOT EXISTS "client_operation_id" uuid;

CREATE UNIQUE INDEX IF NOT EXISTS set_log_client_operation_id_idx
  ON "set_log" ("client_operation_id")
  WHERE "client_operation_id" IS NOT NULL;
