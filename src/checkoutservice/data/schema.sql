CREATE TABLE checkout_orders(checkout_id CHAR(36) PRIMARY KEY, transaction_id CHAR(36), tracking_id VARCHAR(20));
CREATE TABLE ordered_items(item_id SERIAL PRIMARY KEY, order_id CHAR(36) REFERENCES checkout_orders(checkout_id) ON DELETE CASCADE, product_id VARCHAR(10), quantity INT, price TEXT);

INSERT INTO checkout_orders VALUES ("84a2f6aa-b658-4f13-98ec-0b9f14318808", "9b2eedf3-711c-4f78-b161-07b3522ccdd2", "AA-31000-150000000");
INSERT INTO ordered_items VALUES (0, "84a2f6aa-b658-4f13-98ec-0b9f14318808", "66VCHSJNUP", 1, "{\"currency code\": \"USD\", \"units\": 18, \"nanos\": 990000000}")
INSERT INTO ordered_items VALUES (0, "84a2f6aa-b658-4f13-98ec-0b9f14318808", "OLJCESPC7Z", 1, "{\"currency code\": \"USD\", \"units\": 19, \"nanos\": 990000000}")
INSERT INTO ordered_items VALUES (0, "84a2f6aa-b658-4f13-98ec-0b9f14318808", "9SIQT8TOJO", 1, "{\"currency code\": \"USD\", \"units\": 5, \"nanos\": 490000000}")
