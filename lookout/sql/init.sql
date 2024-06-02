CREATE TABLE users (
    id serial PRIMARY KEY NOT NULL,
    name text NOT NULL,
    user_uuid uuid NOT NULL,
    pass_uuid uuid NOT NULL
);
INSERT INTO users(name,user_uuid,pass_uuid) VALUES('OxOOO5cO77','f49f117c-ab06-3794-d7b1-18d12ab88826','94cbea8d-4b8e-42c2-d342-83484b0b4d91');
