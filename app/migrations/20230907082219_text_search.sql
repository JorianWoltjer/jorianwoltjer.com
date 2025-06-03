CREATE FUNCTION plain_markdown(markdown TEXT) returns TEXT LANGUAGE sql IMMUTABLE AS $$
SELECT
    regexp_replace(markdown, '[*`\\#\[\]()]', '', 'g') $$;

ALTER TABLE
    posts
ADD
    COLUMN plain_text TEXT GENERATED ALWAYS AS (plain_markdown(markdown)) STORED,
ADD
    COLUMN ts tsvector GENERATED ALWAYS AS (
        setweight(
            to_tsvector('english', title),
            'A'
        ) || setweight(
            to_tsvector('english', description),
            'B'
        ) || setweight(
            to_tsvector('english', plain_markdown(markdown)),
            'C'
        )
    ) STORED;