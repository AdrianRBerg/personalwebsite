-- Add up migration script here
CREATE TABLE blog_posts (
    id SERIAL PRIMARY KEY not null,
    title varchar(70) not null,
    body text not null,
    lang char(2) not null,
    short_desc varchar(300) not null,
    date date not null
)