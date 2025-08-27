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

INSERT INTO vibes (name, vibe_group_id)
VALUES ("spring", 1),
       ("summer", 1),
       ("autumn", 1),
       ("winter", 1),
       ("rain", 2),
       ("sunny", 2),
       ("morning", 3),
       ("night", 3)
;

INSERT INTO track_pointers (path)
VALUES ("/home/kt345/Documents/my_workspace/vibing/resource/Glorious_morning.mp3"),
       ("/home/kt345/Documents/my_workspace/vibing/resource/MorningRain.mp3"),
       ("/home/kt345/Documents/my_workspace/vibing/resource/Ocean.mp3"),
       ("/home/kt345/Documents/my_workspace/vibing/resource/Rain.mp3"),
       ("/home/kt345/Documents/my_workspace/vibing/resource/TownNight.mp3"),
       ("/home/kt345/Documents/my_workspace/vibing/resource/Summertime.mp3")
;

INSERT INTO track_vibes (track_id, vibe_id)
VALUES (1, 7),
       (2, 7),
       (2, 5),
       (3, 2),
       (4, 5),
       (5, 8),
       (6, 2),
       (6, 6)
;