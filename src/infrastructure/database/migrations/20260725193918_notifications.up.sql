-- Add up migration script here
create table
    notification (
        id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
        target varchar(255) not null,
        url varchar(255),
        target_user varchar(255),
        active boolean,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );