CREATE TABLE IF NOT EXISTS "user"."tbl_users" (
        pk_user_id varchar(15) NOT NULL UNIQUE PRIMARY KEY,
        pagination_id BIGSERIAL NOT NULL UNIQUE,
        username varchar(50) UNIQUE NOT NULL,
        email varchar(50) DEFAULT NULL,
        password varchar(100) DEFAULT NULL,
        image text,
        is_deleted bool NOT NULL DEFAULT false,
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
