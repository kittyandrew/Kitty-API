CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  email TEXT NOT NULL,
  age NUMERIC NOT NULL,
  active BOOLEAN NOT NULL DEFAULT 't',
  picture TEXT NOT NULL
)
