CREATE TABLE users (
  uuid VARCHAR(255) PRIMARY KEY,
  username VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL
);

CREATE TABLE user_history (
  uuid VARCHAR(255) PRIMARY KEY,
  anime_slug VARCHAR(255) NOT NULL,
  episode_num INT NOT NULL,
  episode_slug VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

  user_uuid VARCHAR(255) NOT NULL,

  CONSTRAINT fk_user_history_user
    FOREIGN KEY (user_uuid)
    REFERENCES users(uuid)
    ON DELETE CASCADE
);

CREATE INDEX idx_user_history_user
  ON user_history(user_uuid);
