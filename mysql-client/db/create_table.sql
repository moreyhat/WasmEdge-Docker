CREATE TABLE IF NOT EXISTS mysql.orders (
    order_id INT,
    product_id INT,
    amount FLOAT,
    shipping FLOAT,
    tax FLOAT,
    shipping_address VARCHAR(256)
);