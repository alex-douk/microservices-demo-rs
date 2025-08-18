CREATE TABLE orders (tracking_id VARCHAR(20) PRIMARY KEY, street_address VARCHAR(512), city VARCHAR(128), state VARCHAR(128), country VARCHAR(128), zip_code INT);
-- Insert EPFL's address as an example into the table (the tracking order is well defined)
INSERT INTO orders VALUES ("AA-31000-150000000", "Route Cantonale", "Lausanne", "Vaud", "Switzerland", 1015);
