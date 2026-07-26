-- Add up migration script here
create table summary
(
    id         INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    public_id  UUID unique          default gen_random_uuid(),
    ai_summary_id INT CONSTRAINT FK_AI_SUMMARY REFERENCES AI_SUMMARY(id),
    title      VARCHAR(255),
    content    TEXT        not null,
    metadata   JSON                 default '{}'::json,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

create index idx_summary_created_at on summary (created_at desc);