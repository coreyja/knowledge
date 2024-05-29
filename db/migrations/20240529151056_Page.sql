-- Ensure the UUID extension is enabled
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE Page (
    page_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id Uuid UNIQUE,
    url_id Uuid UNIQUE,
    url VARCHAR(255) NOT NULL
);