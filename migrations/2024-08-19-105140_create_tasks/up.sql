-- Your SQL goes here

CREATE TABLE "tasks"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"title" TEXT NOT NULL,
	"deadline" DATE,
	"category" JSONB NOT NULL,
	"project_id" INT4,
	FOREIGN KEY ("project_id") REFERENCES "projects"("id")
);

