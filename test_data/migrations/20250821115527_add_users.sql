CREATE TABLE "users"
(
    "telegram_id" bigint UNIQUE PRIMARY KEY NOT NULL,
    "is_bot"      bool                      NOT NULL,
    "first_name"  varchar                   NOT NULL,
    "username"    varchar,
    "nickname"    varchar
);
