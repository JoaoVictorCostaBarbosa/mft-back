-- Add migration script here

CREATE TYPE role_enum AS ENUM ('admin', 'user');

CREATE TYPE exercise_type_enum AS ENUM ('strength', 'cardio', 'flexibility', 'balance');

CREATE TYPE equipment_enum AS ENUM (
  'barbell','dumbbell','machine','bodyweight','kettlebell','resistance_band','other'
);

CREATE TYPE muscle_group_enum AS ENUM (
  'chest','back','shoulders','arms','legs','core','full_body','other'
);

CREATE TYPE set_type_enum AS ENUM ('warmup', 'working', 'drop', 'failure');

CREATE TYPE day_of_week_enum AS ENUM (
  'monday','tuesday','wednesday','thursday','friday','saturday','sunday'
);

CREATE TYPE routine_mode_enum AS ENUM ('weekly', 'sequential');

CREATE TYPE routine_item_type_enum AS ENUM ('workout', 'rest');
