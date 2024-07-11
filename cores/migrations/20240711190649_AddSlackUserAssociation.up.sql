-- Add migration script here
CREATE TABLE
  SlackUserLink (
    slack_user_link_id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    slack_user_id VARCHAR(255) NOT NULL,
    slack_username VARCHAR(255) NOT NULL
  );

CREATE TABLE
  SlackUserAssociation (
    slack_user_association_id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    user_id UUID REFERENCES Users (user_id) NOT NULL,
    slack_user_link_id UUID REFERENCES SlackUserLink (slack_user_link_id) NOT NULL,
    slack_user_id VARCHAR(255) NOT NULL,
    slack_username VARCHAR(255) NOT NULL
  );
