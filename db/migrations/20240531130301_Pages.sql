CREATE TABLE Pages (
    page_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES Users (user_id) NOT NULL,
    url VARCHAR(255) NOT NULL,
    UNIQUE (user_id, url)
);

