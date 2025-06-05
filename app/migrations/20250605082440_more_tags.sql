INSERT INTO
    tags (name, color)
VALUES
    ('Social Engineering', 'green'),
    ('CSS', 'yellow'),
    ('Race Condition', 'red'),
    ('Deserialization', 'red'),
    ('XS-Leak', 'blue'),
    ('SSTI', 'green'),
    ('Windows', 'yellow'),
    ('AES', 'yellow'),
    ('RSA', 'red'),
    ('XOR', 'blue'),
    ('Unintended', 'blue')
ON CONFLICT (name) DO NOTHING;