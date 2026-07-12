-- Add up migration script here
CREATE TABLE feeds
(
    id         INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title      VARCHAR(255)  NOT NULL,
    url        VARCHAR(2048) NOT NULL UNIQUE,
    feed_type  VARCHAR(50)   NOT NULL,
    is_active  BOOLEAN       NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);

CREATE TABLE articles
(
    id             BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    feed_id        INT         NOT NULL REFERENCES feeds (id) ON DELETE CASCADE,
    guid           TEXT        NOT NULL,
    url            VARCHAR(2048),
    title          TEXT        NOT NULL,
    content        TEXT,
    raw_extensions JSONB       NOT NULL DEFAULT '{}'::jsonb,
    published_at   TIMESTAMPTZ NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT uq_feed_guid UNIQUE (feed_id, guid)
);

CREATE INDEX idx_articles_published_at_desc ON articles (published_at DESC);