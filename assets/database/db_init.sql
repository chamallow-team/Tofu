DROP TABLE IF EXISTS plugin_links;
DROP TABLE IF EXISTS guilds, users, plugins;

CREATE TABLE users
(
    id           VARCHAR(36) NOT NULL,
    accepted_tos BOOLEAN     NOT NULL DEFAULT FALSE,
    last_seen    DATE        NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id)
);

CREATE TABLE plugins
(
    id         CHAR(36) NOT NULL,
    package_id INT      NOT NULL,

    PRIMARY KEY (id)
);

CREATE TABLE guilds
(
    id            VARCHAR(36) NOT NULL,
    last_activity DATE        NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id)
);

CREATE TABLE plugin_links
(
    plugin_id CHAR(36)    NOT NULL,
    guild_id  VARCHAR(36) NOT NULL,

    PRIMARY KEY (guild_id, plugin_id),

    FOREIGN KEY (plugin_id) REFERENCES plugins (id),
    FOREIGN KEY (guild_id) REFERENCES guilds (id)
);

DELIMITER //
CREATE OR REPLACE PROCEDURE clear_inactive_users()
BEGIN
    DELETE
    FROM users
    WHERE last_seen < DATE_SUB(NOW(), INTERVAL 30 DAY)
       OR (last_seen < DATE_SUB(NOW(), INTERVAL 7 DAY) AND NOT accepted_tos);
end //
DELIMITER ;

DELIMITER //
CREATE OR REPLACE PROCEDURE clear_inactive_guilds()
BEGIN
    DELETE
    FROM guilds
    WHERE last_activity < DATE_SUB(NOW(), INTERVAL 30 DAY);
end //
DELIMITER ;


DELIMITER //
CREATE OR REPLACE PROCEDURE ensure_user(user_id VARCHAR(36))
BEGIN
    UPDATE users
    SET last_seen = NOW()
    WHERE id = user_id;

    IF ROW_COUNT() = 0 THEN
        INSERT IGNORE INTO users (id, accepted_tos, last_seen)
        VALUES (user_id, false, NOW());
    END IF;
end //
DELIMITER ;

DELIMITER //
CREATE OR REPLACE PROCEDURE ensure_guild(guild_id VARCHAR(36))
BEGIN
    UPDATE guilds
    SET last_activity = NOW()
    WHERE id = guild_id;

    IF ROW_COUNT() = 0 THEN
        INSERT IGNORE INTO guilds (id, last_activity)
            VALUE (guild_id, NOW());
    END IF;
END //
DELIMITER ;