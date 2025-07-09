-- User Registration Function
CREATE OR REPLACE FUNCTION sp_register_user(
    p_username VARCHAR,
    p_email VARCHAR,
    p_passwordhash VARCHAR
)
RETURNS INTEGER AS $$
DECLARE
    new_user_id INTEGER;
BEGIN
    INSERT INTO Users (Username, Email, PasswordHash, CreatedAt, UpdatedAt)
    VALUES (p_username, p_email, p_passwordhash, 
            CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Bangkok',
            CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Bangkok')
    RETURNING Id INTO new_user_id;

    RETURN new_user_id;
END;
$$ LANGUAGE plpgsql;

-- User Login Function
CREATE OR REPLACE FUNCTION sp_login_user(p_email VARCHAR)
RETURNS TABLE (Id INT, Username VARCHAR, Email VARCHAR, PasswordHash VARCHAR, CreatedAt TIMESTAMPTZ) AS $$
BEGIN
    RETURN QUERY
    SELECT Id, Username, Email, PasswordHash, CreatedAt
    FROM Users
    WHERE Email = p_email;
END;
$$ LANGUAGE plpgsql;

-- Get User by ID
CREATE OR REPLACE FUNCTION sp_get_user_by_id(p_user_id INT)
RETURNS TABLE (Id INT, Username VARCHAR, Email VARCHAR, CreatedAt TIMESTAMPTZ) AS $$
BEGIN
    RETURN QUERY
    SELECT Id, Username, Email, CreatedAt
    FROM Users
    WHERE Id = p_user_id;
END;
$$ LANGUAGE plpgsql;

-- Create Note
CREATE OR REPLACE FUNCTION sp_create_note(
    p_title VARCHAR,
    p_content TEXT,
    p_user_id INT
)
RETURNS INTEGER AS $$
DECLARE
    new_note_id INTEGER;
BEGIN
    INSERT INTO Notes (Title, Content, UserId, CreatedAt, UpdatedAt)
    VALUES (p_title, p_content, p_user_id,
            CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Bangkok',
            CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Bangkok')
    RETURNING Id INTO new_note_id;

    RETURN new_note_id;
END;
$$ LANGUAGE plpgsql;

-- Create or Update Note
CREATE OR REPLACE FUNCTION sp_create_or_update_note(
    p_note_id INT,
    p_title VARCHAR,
    p_content TEXT,
    p_user_id INT
)
RETURNS TABLE (NoteId INT, Operation TEXT) AS $$
BEGIN
    IF p_note_id IS NULL OR p_note_id = 0 THEN
        INSERT INTO Notes (Title, Content, UserId, CreatedAt)
        VALUES (p_title, p_content, p_user_id,
                CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Bangkok')
        RETURNING Id AS NoteId, 'created'::TEXT AS Operation;
    ELSE
        UPDATE Notes
        SET Title = p_title,
            Content = p_content,
            UpdatedAt = CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Bangkok'
        WHERE Id = p_note_id AND UserId = p_user_id;

        IF FOUND THEN
            RETURN QUERY SELECT p_note_id, 'updated';
        ELSE
            RETURN QUERY SELECT NULL::INT, 'not_found';
        END IF;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Get User Notes
CREATE OR REPLACE FUNCTION sp_get_user_notes(p_user_id INT)
RETURNS TABLE (Id INT, Title VARCHAR, Content TEXT, CreatedAt TIMESTAMPTZ, UpdatedAt TIMESTAMPTZ) AS $$
BEGIN
    RETURN QUERY
    SELECT Id, Title, Content, CreatedAt, UpdatedAt
    FROM Notes
    WHERE UserId = p_user_id
    ORDER BY UpdatedAt DESC;
END;
$$ LANGUAGE plpgsql;

-- Get Note by ID
CREATE OR REPLACE FUNCTION sp_get_note_by_id(p_note_id INT, p_user_id INT)
RETURNS TABLE (Id INT, Title VARCHAR, Content TEXT, CreatedAt TIMESTAMPTZ, UpdatedAt TIMESTAMPTZ) AS $$
BEGIN
    RETURN QUERY
    SELECT Id, Title, Content, CreatedAt, UpdatedAt
    FROM Notes
    WHERE Id = p_note_id AND UserId = p_user_id;
END;
$$ LANGUAGE plpgsql;

-- Update Note
CREATE OR REPLACE FUNCTION sp_update_note(
    p_note_id INT,
    p_title VARCHAR,
    p_content TEXT,
    p_user_id INT
)
RETURNS INTEGER AS $$
BEGIN
    UPDATE Notes
    SET Title = p_title,
        Content = p_content,
        UpdatedAt = CURRENT_TIMESTAMP AT TIME ZONE 'Asia/Bangkok'
    WHERE Id = p_note_id AND UserId = p_user_id;

    RETURN FOUND::INT;  -- 1 if updated, 0 if not
END;
$$ LANGUAGE plpgsql;

-- Delete Note
CREATE OR REPLACE FUNCTION sp_delete_note(p_note_id INT, p_user_id INT)
RETURNS INTEGER AS $$
BEGIN
    DELETE FROM Notes
    WHERE Id = p_note_id AND UserId = p_user_id;

    RETURN FOUND::INT;
END;
$$ LANGUAGE plpgsql;

-- Search Notes
CREATE OR REPLACE FUNCTION sp_search_notes(p_user_id INT, p_search_term VARCHAR DEFAULT NULL)
RETURNS TABLE (Id INT, Title VARCHAR, Content TEXT, CreatedAt TIMESTAMPTZ, UpdatedAt TIMESTAMPTZ) AS $$
BEGIN
    RETURN QUERY
    SELECT Id, Title, Content, CreatedAt, UpdatedAt
    FROM Notes
    WHERE UserId = p_user_id
    AND (
        p_search_term IS NULL OR
        Title ILIKE '%' || p_search_term || '%' OR
        Content ILIKE '%' || p_search_term || '%'
    )
    ORDER BY UpdatedAt DESC;
END;
$$ LANGUAGE plpgsql;
