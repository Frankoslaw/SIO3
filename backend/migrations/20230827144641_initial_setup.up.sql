-- Add up migration script here
CREATE TABLE IF NOT EXISTS Users(
    id SERIAL PRIMARY KEY NOT NULL,
    username VARCHAR(30) NOT NULL UNIQUE,
    password VARCHAR(30) NOT NULL,
    email VARCHAR(30) NOT NULL,
    verified BOOLEAN NOT NULL
);
CREATE TABLE IF NOT EXISTS Competitions(
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(20) NOT NULL UNIQUE,
    owner_id INT NOT NULL,
    end_date DATE DEFAULT (CURRENT_DATE + 1) NOT NULL,
    CONSTRAINT fk_user_id FOREIGN KEY(owner_id) REFERENCES Users(id)
);
CREATE TABLE IF NOT EXISTS Tasks(
    id SERIAL PRIMARY KEY NOT NULL,
    competition_id INT NOT NULL,
    author_id INT NOT NULL,
    pdf_url VARCHAR(60),
    max_submissions INT NOT NULL,
    CONSTRAINT fk_competition_id FOREIGN KEY(competition_id) REFERENCES Competitions(id),
    CONSTRAINT fk_user_id FOREIGN KEY(author_id) REFERENCES Users(id)
);
CREATE TABLE IF NOT EXISTS Submissions(
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INT NOT NULL,
    task_id INT NOT NULL,
    code VARCHAR(400) NOT NULL,
    CONSTRAINT fk_task_id FOREIGN KEY(task_id) REFERENCES Tasks(id),
    CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES Users(id)
);
CREATE TABLE IF NOT EXISTS Tests(
    id SERIAL PRIMARY KEY NOT NULL,
    task_id INT NOT NULL,
    input INT NOT NULL,
    output INT NOT NULL,
    points INT NOT NULL,
    CONSTRAINT fk_task_id FOREIGN KEY(task_id) REFERENCES Tasks(id)
);
CREATE TABLE IF NOT EXISTS Limits(
    id SERIAL PRIMARY KEY NOT NULL,
    lang VARCHAR(10) NOT NULL,
    cpu INT NOT NULL,
    mem INT NOT NULL,
    task_id INT NOT NULL,
    CONSTRAINT fk_task_id FOREIGN KEY(task_id) REFERENCES Tasks(id)
);
CREATE TABLE IF NOT EXISTS Scores(
    id SERIAL PRIMARY KEY NOT NULL,
    submission_id INT NOT NULL,
    status VARCHAR(30) NOT NULL,
    score INT NOT NULL,
    CONSTRAINT fk_submission_id FOREIGN KEY(submission_id) REFERENCES Submissions(id)
);
CREATE TABLE IF NOT EXISTS TestScores(
    id SERIAL PRIMARY KEY NOT NULL,
    submission_id INT NOT NULL,
    test_id INT NOT NULL,
    passed BOOLEAN NOT NULL,
    error_message TEXT,
    CONSTRAINT fk_submission_id FOREIGN KEY(submission_id) REFERENCES Submissions(id),
    CONSTRAINT fk_test_id FOREIGN KEY(test_id) REFERENCES Tests(id)
);
