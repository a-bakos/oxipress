CREATE TABLE IF NOT EXISTS gb_accounts (
    id serial PRIMARY KEY,
    login VARCHAR(60) NOT NULL,
    password VARCHAR(255) NOT NULL,
    email VARCHAR(100) NOT NULL,
    registered TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP DEFAULT NULL,
    role VARCHAR(20) NOT NULL,
    token VARCHAR(100) DEFAULT NULL
);
