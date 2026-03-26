CREATE TABLE "roosters"
(
    "id"      bigint UNIQUE PRIMARY KEY NOT NULL,
    "chat_id" bigint                    NOT NULL,
    "name"    varchar                   NOT NULL
);
