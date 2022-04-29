DROP TABLE IF EXISTS media;

DROP TABLE IF EXISTS tag;

DROP TABLE IF EXISTS user;

DROP TABLE IF EXISTS post;

DROP TABLE IF EXISTS post_page;

DROP TABLE IF EXISTS post_tag;

-- CREATE UNIQUE INDEX uq_kind_name ON tag ( kind, name );

-- CREATE UNIQUE INDEX uq_user ON user ( username, email );

-- CREATE UNIQUE INDEX uq_post_page ON post_page ( page, post_id, page_id );