-- Add up migration script here
CREATE TABLE IF NOT EXISTS News(
    id SERIAL PRIMARY KEY NOT NULL,
    competition_id INT NOT NULL,
    author_id INT NOT NULL,
    title VARCHAR(30) NOT NULL,
    content VARCHAR(300) NOT NULL,
    CONSTRAINT fk_competition_id FOREIGN KEY(competition_id) REFERENCES Competitions(id),
    CONSTRAINT fk_user_id FOREIGN KEY(author_id) REFERENCES Users(id)
);
