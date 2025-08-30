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
VALUES ("seasonal"), -- id = 1
       ("weather"),  -- id = 2
       ("daytime"),  -- id = 3
       ("mood"),     -- id = 4
       ("event")     -- id = 5
;

INSERT INTO vibes (name, vibe_group_id)
VALUES ("spring", 1),
       ("summer", 1),
       ("autumn", 1),
       ("winter", 1),
       ("sunny", 2),
       ("rainy", 2),
       ("windy", 2),
       ("cloudy", 2),
       ("stormy", 2),
       ("hooty", 2),
       ("coldy", 2),
       ("dawn", 3),
       ("morning", 3),
       ("noon", 3),
       ("afternoon", 3),
       ("dusk", 3),
       ("evening", 3),
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

-- insert sample
INSERT INTO track_vibes (track_id, vibe_id)
VALUES
    (
        (SELECT track_id FROM track_pointers WHERE path LIKE '%/Glorious_morning.mp3'),
        (SELECT vibe_id FROM vibes WHERE name = "morning")
    ),
    (
        (SELECT track_id FROM track_pointers WHERE path LIKE '%/MorningRain.mp3'),
        (SELECT vibe_id FROM vibes WHERE name = "morning")
    ),
    (
        (SELECT track_id FROM track_pointers WHERE path LIKE '%/MorningRain.mp3'),
        (SELECT vibe_id FROM vibes WHERE name = "rainy")
    ),
    (
        (SELECT track_id FROM track_pointers WHERE path LIKE '%/Ocean.mp3'),
        (SELECT vibe_id FROM vibes WHERE name = "summer")
    ),
    (
        (SELECT track_id FROM track_pointers WHERE path LIKE '%/Rain.mp3'),
        (SELECT vibe_id FROM vibes WHERE name = "rainy")
    ),
    (
        (SELECT track_id FROM track_pointers WHERE path LIKE '%/TownNight.mp3'),
        (SELECT vibe_id FROM vibes WHERE name = "evening")
    ),
    (
        (SELECT track_id FROM track_pointers WHERE path LIKE '%/TownNight.mp3'),
        (SELECT vibe_id FROM vibes WHERE name = "night")
    ),
    (
        (SELECT track_id FROM track_pointers WHERE path LIKE '%/Summertime.mp3'),
        (SELECT vibe_id FROM vibes WHERE name = "summer")
    ),
    (
        (SELECT track_id FROM track_pointers WHERE path LIKE '%/Summertime.mp3'),
        (SELECT vibe_id FROM vibes WHERE name = "sunny")
    )
;