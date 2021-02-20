CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  email TEXT NOT NULL,
  age INTEGER NOT NULL,
  active BOOLEAN NOT NULL,
  picture TEXT NOT NULL
);
