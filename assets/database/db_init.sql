DROP TABLE IF EXISTS plugin_verification_examinators;
DROP TABLE IF EXISTS plugin_verifications;
DROP TABLE IF EXISTS plugin_version;
DROP TABLE IF EXISTS plugin_links, plugin_name, plugin_desc;
DROP TABLE IF EXISTS plugin_comments;
DROP TABLE IF EXISTS guild_plugins;
DROP TABLE IF EXISTS guilds, plugins, staff;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS languages;

CREATE TABLE languages
(
    code VARCHAR(3)  NOT NULL,
    name VARCHAR(64) NOT NULL UNIQUE,

    PRIMARY KEY (code)
);

CREATE TABLE users
(
    id                      VARCHAR(36) NOT NULL,
    accepted_tos            BOOLEAN     NOT NULL DEFAULT FALSE,
    last_seen               DATE        NOT NULL DEFAULT NOW(),

    name                    VARCHAR(64),
    email                   VARCHAR(255),

    preferred_language_code VARCHAR(3),

    PRIMARY KEY (id),
    FOREIGN KEY (preferred_language_code) REFERENCES languages (code)
);

CREATE TABLE guilds
(
    id                      VARCHAR(36) NOT NULL,
    last_activity           DATE        NOT NULL DEFAULT NOW(),

    preferred_language_code VARCHAR(3),

    PRIMARY KEY (id),
    FOREIGN KEY (preferred_language_code) REFERENCES languages (code)
);

CREATE TABLE plugins
(
    id         CHAR(36)     NOT NULL,
    author_id  VARCHAR(36)  NOT NULL,
    name       VARCHAR(128) NOT NULL UNIQUE,

    is_no_code BOOLEAN      NOT NULL,
    private    BOOLEAN      NOT NULL DEFAULT true,

    PRIMARY KEY (id),
    FOREIGN KEY (author_id) REFERENCES users (id)
);

CREATE TABLE staff
(
    staff_id     CHAR(36)     NOT NULL,
    permissions  BIGINT       NOT NULL,
    name         VARCHAR(64)  NOT NULL,

    login        VARCHAR(32)  NOT NULL UNIQUE,
    password_has VARCHAR(255) NOT NULL,

    PRIMARY KEY (staff_id)
);


CREATE TABLE plugin_name
(
    plugin_id     CHAR(36)    NOT NULL,
    language_code VARCHAR(3)  NOT NULL,
    name          VARCHAR(64) NOT NULL UNIQUE,

    PRIMARY KEY (plugin_id, language_code),
    FOREIGN KEY (plugin_id) REFERENCES plugins (id),
    FOREIGN KEY (language_code) REFERENCES languages (code)
);

CREATE TABLE plugin_desc
(
    plugin_id     CHAR(36)   NOT NULL,
    language_code VARCHAR(3) NOT NULL,

    PRIMARY KEY (plugin_id, language_code),
    FOREIGN KEY (plugin_id) REFERENCES plugins (id),
    FOREIGN KEY (language_code) REFERENCES languages (code)
);

CREATE TABLE plugin_links
(
    link_id   INTEGER     NOT NULL AUTO_INCREMENT,
    plugin_id CHAR(36)    NOT NULL,
    name      VARCHAR(64) NOT NULL,
    url       VARCHAR(64) NOT NULL,
    validated BOOLEAN     NOT NULL DEFAULT false,

    PRIMARY KEY (link_id),
    FOREIGN KEY (plugin_id) REFERENCES plugins (id)
);

CREATE TABLE plugin_comments
(
    author_id VARCHAR(36)   NOT NULL,
    plugin_id VARCHAR(36)   NOT NULL,
    comment   VARCHAR(1024) NOT NULL,
    verified  BOOLEAN       NOT NULL DEFAULT false,
    likes     INTEGER       NOT NULL DEFAULT 0,

    PRIMARY KEY (author_id, plugin_id),
    FOREIGN KEY (author_id) REFERENCES users (id),
    FOREIGN KEY (plugin_id) REFERENCES plugins (id)
);

CREATE TABLE guild_plugins
(
    plugin_id CHAR(36)    NOT NULL,
    guild_id  VARCHAR(36) NOT NULL,
    author_id VARCHAR(36) NOT NULL,

    enabled   BOOLEAN     NOT NULL DEFAULT false,

    PRIMARY KEY (guild_id, plugin_id),

    FOREIGN KEY (plugin_id) REFERENCES plugins (id),
    FOREIGN KEY (guild_id) REFERENCES guilds (id),
    FOREIGN KEY (author_id) REFERENCES users (id)
);

CREATE TABLE plugin_version
(
    plugin_id  CHAR(36)    NOT NULL,
    version    VARCHAR(10) NOT NULL,
    created_at DATETIME    NOT NULL DEFAULT now(),
    edited_at  DATETIME,
    submitted  BOOLEAN     NOT NULL DEFAULT false,

    PRIMARY KEY (version, plugin_id),
    FOREIGN KEY (plugin_id) REFERENCES plugins (id)
);

CREATE TABLE plugin_verifications
(
    verification_id CHAR(36)    NOT NULL,
    plugin_id       CHAR(36)    NOT NULL,
    version_id      VARCHAR(10) NOT NULL,
    responsable_id  CHAR(36)    NOT NULL,

    PRIMARY KEY (verification_id),
    FOREIGN KEY (plugin_id) REFERENCES plugins (id),
    FOREIGN KEY (plugin_id, version_id) REFERENCES plugin_version (plugin_id, version)
);

CREATE TABLE plugin_verification_examinators
(
    staff_id        CHAR(36) NOT NULL,
    verification_id CHAR(36) NOT NULL,

    validation      BOOLEAN  NOT NULL DEFAULT false,
    already_replied BOOLEAN  NOT NULL DEFAULT false,

    PRIMARY KEY (staff_id, verification_id),
    FOREIGN KEY (staff_id) REFERENCES staff (staff_id),
    FOREIGN KEY (verification_id) REFERENCES plugin_verifications (verification_id)
);


#
#
#   PROCEDURES
#
#

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