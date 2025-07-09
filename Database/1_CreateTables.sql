-- Connect to the NotesDB database
-- (In psql, switch databases before running this script, or run with -d NotesDB)

-- Create Users table
CREATE TABLE IF NOT EXISTS Users (
    Id SERIAL PRIMARY KEY,
    Username VARCHAR(100) NOT NULL UNIQUE,
    Email VARCHAR(255) NOT NULL UNIQUE,
    PasswordHash VARCHAR(255) NOT NULL,
    CreatedAt TIMESTAMPTZ DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Bangkok'),
    UpdatedAt TIMESTAMPTZ DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Bangkok')
);

-- Create Notes table
CREATE TABLE IF NOT EXISTS Notes (
    Id SERIAL PRIMARY KEY,
    Title VARCHAR(255) NOT NULL,
    Content TEXT,
    UserId INT NOT NULL,
    CreatedAt TIMESTAMPTZ DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Bangkok'),
    UpdatedAt TIMESTAMPTZ DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Bangkok'),
    CONSTRAINT fk_user FOREIGN KEY(UserId) REFERENCES Users(Id) ON DELETE CASCADE
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS IX_Notes_UserId ON Notes(UserId);
CREATE INDEX IF NOT EXISTS IX_Notes_Title ON Notes(Title);
CREATE INDEX IF NOT EXISTS IX_Users_Email ON Users(Email);
CREATE INDEX IF NOT EXISTS IX_Users_Username ON Users(Username);
