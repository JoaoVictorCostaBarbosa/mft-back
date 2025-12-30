-- Add migration script here

CREATE TABLE "refresh_token" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid NOT NULL, 
  "hash" text NOT NULL UNIQUE,
  "expires_at" timestamptz NOT NULL,
  "revoked" boolean NOT NULL DEFAULT false,
  "created_at" timestamptz NOT NULL DEFAULT current_timestamp,

  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON UPDATE CASCADE ON DELETE CASCADE
);
