CREATE TABLE birds (
  id SERIAL PRIMARY KEY,
  species VARCHAR NOT NULL,
  colors VARCHAR NOT NULL,
  cat_id INT,
  FOREIGN KEY(cat_id) REFERENCES cats(id)
)
