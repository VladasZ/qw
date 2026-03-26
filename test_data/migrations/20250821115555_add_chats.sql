CREATE TYPE "chat_kind" AS ENUM (
    'public',
    'private'
    );

CREATE TABLE "chats"
(
    "telegram_id" bigint UNIQUE PRIMARY KEY NOT NULL,
    "name"        varchar                   NOT NULL,
    "kind"        chat_kind                 NOT NULL
);