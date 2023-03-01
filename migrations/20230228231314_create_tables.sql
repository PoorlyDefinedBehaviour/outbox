CREATE DATABASE IF NOT EXISTS outbox;

CREATE TABLE IF NOT EXISTS entities (
    id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    status INT NOT NULL
);

INSERT INTO entities(id, status) VALUES (1, 0);

CREATE TABLE IF NOT EXISTS operations (
    id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    entity_id INT NOT NULL,
    operation INT NOT NULL,
    status INT NOT NULL,
    CONSTRAINT fk_entity_id
    FOREIGN KEY(entity_id) REFERENCES entities(id)
);