-- Your SQL goes here

-- CREATE TYPE Category AS ENUM ('exaltacion', 'adoracion', 'ministracion');

CREATE TABLE songs (
	id INTEGER NOT NULL PRIMARY KEY ,
	name TEXT NOT NULL,
	category Category NOT NULL,
	interpreter TEXT NOT NULL,
	num_verses INTEGER NOT NULL
);

CREATE TABLE verses (
	id INTEGER NOT NULL PRIMARY KEY,
	content TEXT NOT NULL,
	song_id INTEGER NOT NULL,
	foreign key (song_id) references songs(id)
)



