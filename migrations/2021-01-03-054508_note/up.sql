CREATE TABLE notes (
  id SERIAL PRIMARY KEY,
  uuid VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  description TEXT NOT NULL,
  create_time VARCHAR NOT NULL,
  last_update_time VARCHAR NOT NULL,
  delete_time VARCHAR
)
