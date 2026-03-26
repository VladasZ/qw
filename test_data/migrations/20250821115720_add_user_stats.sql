CREATE TABLE "user_stats"
(
    "user_id"  bigint  NOT NULL,
    "chat_id"  bigint  NOT NULL,
    "messages" integer NOT NULL,
    "kto"      integer NOT NULL,
    "llm"      integer NOT NULL,
    "commands" integer NOT NULL,
    PRIMARY KEY ("user_id", "chat_id")
);

ALTER TABLE "user_stats"
    ADD FOREIGN KEY ("user_id") REFERENCES "users" ("telegram_id");

ALTER TABLE "user_stats"
    ADD FOREIGN KEY ("chat_id") REFERENCES "chats" ("telegram_id");
