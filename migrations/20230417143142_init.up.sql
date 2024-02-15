-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS notes (
        id BIGINT PRIMARY KEY NOT NULL AUTO_INCREMENT,
        -- title VARCHAR(255) NOT NULL UNIQUE,
        summary TEXT NOT NULL,
        priority VARCHAR(255) NOT NULL DEFAULT 'low',
        status VARCHAR(255) NOT NULL DEFAULT 'created',
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
    );