CREATE TABLE IF NOT EXISTS ordered_food (
    id SERIAL PRIMARY KEY,
    id_food INT NOT NULL,
    amount INT NOT NULL,
    UNIQUE (id_food)
);

CREATE TABLE IF NOT EXISTS food (
    id SERIAL PRIMARY KEY,
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
    id_restaurant INT NOT NULL,
    id_ordered_food INT NOT NULL,
    FOREIGN KEY (id_client) REFERENCES client (id),
    FOREIGN KEY (id_restaurant) REFERENCES restaurant (id),
    FOREIGN KEY (id_ordered_food) REFERENCES ordered_food (id)
);

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint
        WHERE conname = 'fk_ordered_food_food'
    ) THEN
ALTER TABLE ordered_food ADD CONSTRAINT fk_ordered_food_food FOREIGN KEY (id_food) REFERENCES food (id);
END IF;
END
$$;

