CREATE TABLE "users" (
    "id" SERIAL PRIMARY KEY,
    "email" VARCHAR NOT NULL,
    "name" VARCHAR NOT NULL,
    "age" INT NOT NULL,
    "active" BOOLEAN NOT NULL DEFAULT FALSE
);
