-- Add migration script here

CREATE TABLE "exercise" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid,
  "name" varchar(100) NOT NULL,
  "exercise_type" exercise_type_enum NOT NULL,
  "equipment" equipment_enum NOT NULL,
  "muscle_group" muscle_group_enum NOT NULL,
  "created_at" timestamptz DEFAULT current_timestamp NOT NULL,
  "deleted_at" timestamptz,

  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON UPDATE CASCADE
);
