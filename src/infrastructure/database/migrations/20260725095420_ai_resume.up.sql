-- Add up migration script here
CREATE TABLE AI_CLASSIFICATION
(
    id         INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    content    JSON        NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE AI_SUMMARY
(
    id                   INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    AI_CLASSIFICATION_ID INT
        CONSTRAINT FK_AI_CLASSIFICATION REFERENCES AI_CLASSIFICATION (id) NOT NULL,
    content              JSON                                             NOT NULL,
    created_at           TIMESTAMPTZ                                      NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ                                      NOT NULL DEFAULT NOW()
)