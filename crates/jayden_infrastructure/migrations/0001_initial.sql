DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    email VARCHAR(50) NOT NULL,
    bio VARCHAR(500),
    image TEXT,
    password VARCHAR(100),
    public_key VARCHAR(100)
);


INSERT INTO users (username, email, bio, image, password, public_key)
VALUES
    ('john_doe', 'john@example.com', 'A passionate individual.', 'path/to/image1.jpg', 'hashed_password1', 'public_key_value1'),
    ('jane_smith', 'jane@example.com', 'Enthusiastic learner.', 'path/to/image2.jpg', 'hashed_password2', 'public_key_value2'),
    ('bob_jones', 'bob@example.com', 'Tech enthusiast.', 'path/to/image3.jpg', 'hashed_password3', 'public_key_value3');
