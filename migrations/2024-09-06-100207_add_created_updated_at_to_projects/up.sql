-- Your SQL goes here
ALTER TABLE "projects" ADD COLUMN "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE "projects" ADD COLUMN "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;

SELECT diesel_manage_updated_at('projects');


