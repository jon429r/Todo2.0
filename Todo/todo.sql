CREATE TABLE projects (
    id INTEGER PRIMARY KEY,
    name TEXT,
    working_directory TEXT,
    project_link TEXT,
    description TEXT
);

CREATE TABLE finished_projects (
    id INTEGER PRIMARY KEY,
    name TEXT,
    working_directory TEXT,
    project_link TEXT,
    description TEXT
);

CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username TEXT,
    password TEXT
);

CREATE TABLE user_projects (
    id INTEGER PRIMARY KEY,
    user_id INTEGER,
    project_id INTEGER,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

