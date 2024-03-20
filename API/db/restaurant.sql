CREATE TABLE IF NOT EXISTS ordered_food (
    id SERIAL PRIMARY KEY,
    id_order INT NOT NULL,
    id_food INT NOT NULL,
    amount INT NOT NULL,
    UNIQUE (id_food)
);

CREATE TABLE IF NOT EXISTS food (
    id SERIAL PRIMARY KEY,
    id_restaurant INT NOT NULL,
    category INT NOT NULL,
    food_name VARCHAR(255) NOT NULL,
    food_price INT NOT NULL
);

CREATE TABLE IF NOT EXISTS restaurant (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    zip_code VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS client (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS "order" (
    id SERIAL PRIMARY KEY,
    id_client INT NOT NULL,
    id_ordered_food SERIAL NOT NULL
);


