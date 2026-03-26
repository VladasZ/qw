CREATE TABLE "saved_responses"
(
    "user_id"  bigint  NOT NULL,
    "chat_id"  bigint  NOT NULL,
    "request"  varchar NOT NULL,
    "response" varchar NOT NULL,
    PRIMARY KEY ("user_id", "chat_id", "request")
);

ALTER TABLE "saved_responses"
    ADD FOREIGN KEY ("user_id") REFERENCES "users" ("telegram_id");

ALTER TABLE "saved_responses"
    ADD FOREIGN KEY ("chat_id") REFERENCES "chats" ("telegram_id");