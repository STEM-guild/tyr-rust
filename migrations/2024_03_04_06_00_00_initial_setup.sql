CREATE TABLE migrations (
                            id SERIAL PRIMARY KEY,
                            name VARCHAR(255) NOT NULL,
                            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE guilds (
                        id BIGINT PRIMARY KEY,
                        name VARCHAR(255) NOT NULL,
                        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE guild_config_options (
                                      id SERIAL PRIMARY KEY,
                                      guild_id BIGINT NOT NULL,
                                      name VARCHAR(255) NOT NULL,
                                      value TEXT NOT NULL,
                                      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE users (
                       id BIGINT PRIMARY KEY,
                       guild_id BIGINT NOT NULL,
                       name VARCHAR(255) NOT NULL,
                       created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE user_infractions (
                                  id BIGINT PRIMARY KEY,
                                  user_id BIGINT NOT NULL,
                                  executor_id BIGINT NOT NULL,
                                  type VARCHAR(255) NOT NULL,
                                  reason TEXT NOT NULL,
                                  duration BIGINT,
                                  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE user_infractions (
                                  id BIGINT PRIMARY KEY,
                                  user_id BIGINT NOT NULL,
                                  executor_id BIGINT NOT NULL,
                                  type VARCHAR(255) NOT NULL,
                                  reason TEXT NOT NULL,
                                  duration BIGINT,
                                  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE user_thanks (
                             id BIGINT PRIMARY KEY,
                             guild_id BIGINT NOT NULL,
                             user_id BIGINT NOT NULL,
                             thank_target_id BIGINT NOT NULL,
                             channel_id BIGINT,
                             thread_id BIGINT,
                             message_id BIGINT,
                             created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE mod_actions (
                             id BIGINT PRIMARY KEY,
                             guild_id BIGINT NOT NULL,
                             user_id BIGINT NOT NULL,
                             type VARCHAR(255) NOT NULL,
                             reason TEXT NOT NULL,
                             created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO migrations (name) VALUES ('2024_03_04_06_00_00_initial_setup.sql');