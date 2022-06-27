CREATE TABLE animals (
  id SERIAL PRIMARY KEY,
  name VARCHAR,
  is_fluffy BOOLEAN,
  date_created TIMESTAMP WITHOUT TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC')
);

INSERT INTO animals(name, is_fluffy) VALUES ('doge', true);

INSERT INTO animals(name, is_fluffy) VALUES ('bingus', false);
