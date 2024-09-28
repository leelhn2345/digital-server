CREATE TABLE tele_chatrooms (
  id bigint NOT NULL PRIMARY KEY,
  title text,
  is_group boolean NOT NULL,
  joined_at timestamptz NOT NULL
);

CREATE TABLE tele_chatlogs (
  id serial PRIMARY KEY,
  chatroom_id bigint REFERENCES tele_chatrooms (id) ON DELETE cascade,
  name text,
  role text NOT NULL,
  content text NOT NULL,
  datetime timestamptz NOT NULL
);
