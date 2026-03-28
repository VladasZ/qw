CREATE TABLE "roosters"
(
    "id"      bigserial UNIQUE PRIMARY KEY NOT NULL,
    "chat_id" bigint                    NOT NULL,
    "name"    varchar                   NOT NULL
);
