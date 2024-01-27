CREATE TABLE IF NOT EXISTS "post"."tbl_posts" (
        pk_post_id varchar(15) NOT NULL UNIQUE PRIMARY KEY,
        user_id varchar(15),
        pagination_id BIGSERIAL NOT NULL UNIQUE,
        title varchar(200) UNIQUE NOT NULL,
        content text DEFAULT NULL,
        thumbnail text,
        is_public bool NOT NULL DEFAULT false,
        created_at BIGINT NOT NULL DEFAULT (
            extract(
                epoch
                from
                    now ()
            ) * 1000
        ),
        updated_at BIGINT NOT NULL DEFAULT (
            extract(
                epoch
                from
                    now ()
            ) * 1000
        )
 );
