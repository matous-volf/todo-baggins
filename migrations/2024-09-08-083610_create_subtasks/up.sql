-- Your SQL goes here


CREATE TABLE "subtasks"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"task_id" INT4 NOT NULL,
	"title" TEXT NOT NULL,
	"is_completed" BOOL NOT NULL,
	"created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY ("task_id") REFERENCES "tasks"("id") ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('subtasks');

