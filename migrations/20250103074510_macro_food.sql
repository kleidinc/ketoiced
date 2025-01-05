-- Add migration script here
CREATE TABLE IF NOT EXISTS "macro_food" (
    macro_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name varchar(25) UNIQUE NOT NULL,
    protein numeric,
    carbohydrates numeric,
    fat numeric,
    kcalories numeric,
    weight numeric
);

INSERT INTO
    "macro_food"(
        name,
        protein,
        carbohydrates,
        fat,
        kcalories,
        weight
    )
VALUES
    ('meat', '12.3', '0.0', '12.3', 253.0, 259.0),
    (
        'bone broth',
        '12.3',
        '0.0',
        '12.3',
        253.0,
        259.0
    );
