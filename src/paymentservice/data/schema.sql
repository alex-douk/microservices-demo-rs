CREATE TABLE payments (transaction_id CHAR(36) PRIMARY KEY, card_type VARCHAR(15), cc_number char(16), expiration_month INT, expiration_year int, cvv CHAR(3));
-- Insert EPFL's address as an example into the table (the tracking order is well defined)
INSERT INTO payments VALUES ("9b2eedf3-711c-4f78-b161-07b3522ccdd2", "mastercard", "5555555555554444", 03, 2030, "737");
