CREATE TABLE IF NOT EXISTS reports_tb (
    id SERIAL PRIMARY KEY,
    operation VARCHAR NOT NULL,
    value_1 INT NOT NULL,
    value_2 INT NOT NULL,
    result INT NOT NULL,
    create_at BIGINT NOT NULL
);