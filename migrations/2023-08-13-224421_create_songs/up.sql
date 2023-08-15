-- Your SQL goes here

CREATE TYPE Category AS ENUM ('exaltacion', 'adoracion', 'ministracion');

CREATE TABLE songs (
	id SERIAL NOT NULL PRIMARY KEY,
	title TEXT NOT NULL,
	category Category NOT NULL,
	interpreter TEXT,
	num_verses INTEGER NOT NULL
);

CREATE TABLE verses (
	id SERIAL NOT NULL PRIMARY KEY,
	content TEXT NOT NULL,
	song_id INTEGER NOT NULL,
	foreign key (song_id) references songs(id)
)



