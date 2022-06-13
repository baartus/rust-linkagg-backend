CREATE TABLE IF NOT EXISTS user_registrations
(
    registration_id SERIAL PRIMARY KEY,
    email VARCHAR(50) NOT NULL UNIQUE,
    username VARCHAR(20) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    registration_hash VARCHAR(255) NOT NULL UNIQUE
);