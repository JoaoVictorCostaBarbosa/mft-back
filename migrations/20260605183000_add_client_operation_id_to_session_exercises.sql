ALTER TABLE "exercise_log"
  ADD COLUMN IF NOT EXISTS "client_operation_id" uuid;

CREATE UNIQUE INDEX IF NOT EXISTS exercise_log_client_operation_id_idx
  ON "exercise_log" ("client_operation_id")
  WHERE "client_operation_id" IS NOT NULL;
