-- create initial tables
CREATE TABLE IF NOT EXISTS track_pointers (
    track_id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS vibe_groups (
    vibe_group_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS vibes (
    vibe_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    vibe_group_id INTEGER NOT NULL,
    FOREIGN KEY (vibe_group_id) REFERENCES vibe_groups(vibe_group_id) ON DELETE CASCADE,
    UNIQUE (name, vibe_group_id)
);

CREATE TABLE IF NOT EXISTS track_vibes (
    track_id INTEGER NOT NULL,
    vibe_id INTEGER NOT NULL,
    PRIMARY KEY (track_id, vibe_id),
    FOREIGN KEY (track_id) REFERENCES track_pointers(track_id) ON DELETE CASCADE,
    FOREIGN KEY (vibe_id) REFERENCES vibes(vibe_id) ON DELETE CASCADE
);

INSERT INTO vibe_groups (name)
VALUES ("seasonal"),
       ("weather"),
       ("daytime"),
       ("mood"),
       ("event")
;
