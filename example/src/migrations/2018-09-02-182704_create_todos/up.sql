CREATE TABLE todos (
  id BIGINT PRIMARY KEY,
  name VARCHAR NOT NULL,
  done BOOLEAN NOT NULL DEFAULT 'f'
);