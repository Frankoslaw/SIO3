-- Add up migration script here
INSERT
    OR IGNORE INTO Users (id, username, password, email, verified)
VALUES (
        1,
        'Frankoslaw',
        'test',
        'franopusz2006@gmail.com',
        TRUE
    ) ON CONFLICT DO NOTHING;
INSERT
    OR REPLACE INTO Competitions (id, name, owner_id)
VALUES (1, 'Frankoslaws death math', 1);
INSERT
    OR IGNORE INTO News (id, competition_id, author_id, title, content)
VALUES (
        1,
        1,
        1,
        'Hello world',
        'Vex to gej'
    ),
    (
        2,
        1,
        1,
        'Test',
        'If yous see it its working correctly'
    ) ON CONFLICT DO NOTHING;
