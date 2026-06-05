-- Add migration script here

CREATE TABLE "set_log" (
  "id" uuid PRIMARY KEY,
  "exercise_log_id" uuid NOT NULL,
  "type" set_type_enum NOT NULL,
  "weight" decimal(5,2) NOT NULL,
  "reps" int NOT NULL,
  "created_at" timestamptz DEFAULT current_timestamp,

  FOREIGN KEY ("exercise_log_id") REFERENCES "exercise_log" ("id") ON UPDATE CASCADE
);
