CREATE TABLE ordered_food (
    id SERIAL PRIMARY KEY,
    id_food INT NOT NULL,
    amount INT NOT NULL,
    UNIQUE (id_food)
);

CREATE TABLE food (
    id SERIAL PRIMARY KEY,
    category INT NOT NULL,
    food_name VARCHAR(255) NOT NULL,
    food_price INT NOT NULL
);

CREATE TABLE restaurant (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    zip_code VARCHAR(255) NOT NULL
);

CREATE TABLE client (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL
);

CREATE TABLE "order" (
    id SERIAL PRIMARY KEY,
    id_client INT NOT NULL,
    id_restaurant INT NOT NULL,
    id_ordered_food INT NOT NULL,
    FOREIGN KEY (id_client) REFERENCES client (id),
    FOREIGN KEY (id_restaurant) REFERENCES restaurant (id),
    FOREIGN KEY (id_ordered_food) REFERENCES ordered_food (id)
);

ALTER TABLE ordered_food
ADD CONSTRAINT fk_ordered_food_food FOREIGN KEY (id_food) REFERENCES food (id);
