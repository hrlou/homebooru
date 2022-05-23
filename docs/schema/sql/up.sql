CREATE TABLE media ( 
	md5                  VARCHAR(127) NOT NULL  PRIMARY KEY  ,
	name                 VARCHAR(255) NOT NULL    ,
	mime                 VARCHAR(127) NOT NULL    
 );

CREATE TABLE tag ( 
	id                   INTEGER NOT NULL  PRIMARY KEY  ,
	kind                 VARCHAR(255) NOT NULL    ,
	name                 VARCHAR(255) NOT NULL    
 );

CREATE TABLE user ( 
	id                   INTEGER NOT NULL  PRIMARY KEY  ,
	username             VARCHAR(127) NOT NULL    ,
	email                VARCHAR(255) NOT NULL    ,
	hash                 VARCHAR(255) NOT NULL    ,
	creation             DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP   
 );

CREATE TABLE post ( 
	id                   INTEGER NOT NULL  PRIMARY KEY  ,
	owner_id             INTEGER NOT NULL    ,
	source               VARCHAR(255)     ,
	title                VARCHAR(255)     ,
	creation             DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP   ,
	FOREIGN KEY ( owner_id ) REFERENCES user( id )  
 );

CREATE TABLE post_page ( 
	page                 INTEGER NOT NULL    ,
	post_id              INTEGER NOT NULL    ,
	page_id              VARCHAR(127) NOT NULL    ,
	CONSTRAINT pk_post_page PRIMARY KEY ( page, post_id ),
	FOREIGN KEY ( post_id ) REFERENCES post( id ) ON DELETE CASCADE ON UPDATE CASCADE,
	FOREIGN KEY ( page_id ) REFERENCES media( md5 )  
 );

CREATE TABLE post_tag ( 
	post_id              INTEGER NOT NULL    ,
	tag_id               INTEGER NOT NULL    ,
	CONSTRAINT pk_post_tag PRIMARY KEY ( post_id, tag_id ),
	FOREIGN KEY ( post_id ) REFERENCES post( id ) ON DELETE CASCADE ON UPDATE CASCADE,
	FOREIGN KEY ( tag_id ) REFERENCES tag( id )  
 );

CREATE UNIQUE INDEX uq_kind_name ON tag ( kind, name );

CREATE UNIQUE INDEX uq_user ON user ( username, email );

CREATE UNIQUE INDEX uq_post_page ON post_page ( page, post_id, page_id );

