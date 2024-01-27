ALTER TABLE "post"."tbl_posts"
ADD CONSTRAINT fk_user_id FOREIGN KEY ("user_id")
REFERENCES "user"."tbl_users" ("pk_user_id") ON DELETE SET NULL;
