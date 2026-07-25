# RSS Dev newsletter

Fetch, store from different RSS feeds. Generate AI-powered summaries and send them to Discord.

This is my first Rust project and is currently in development.

## Table of Contents

- [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
    - [Configuration](#configuration)
- [Features](#features)
    - [RSS](#rss)
        - [Prerequisites](#rss-prerequisites)
        - [Configuration](#rss-configuration)
        - [Feed Types](#feed-types)
    - [Database](#database)
        - [Configuration](#database-configuration)
    - [AI](#ai)
        - [Using Gemini](#using-gemini)
        - [Using Mistral Vibe](#using-mistral-vibe)
    - [Notification](#notification)
        - [Using Discord](#using-discord)

## Getting Started

### Prerequisites

- [Rust](https://rust-lang.org/fr/tools/install/) installed on your system

### Installation

Rename the configuration file from `default-template.toml` to `default.toml`.

## Features

### RSS

The RSS processor fetches and stores newsletter items from TLDR feeds.

#### RSS Prerequisites

To use the RSS processor, you need a **database**. Currently, only PostgreSQL is supported.

#### RSS Configuration

**Global configuration**:

| Field    | Description                            | Default |
|----------|----------------------------------------|---------|
| `enable` | If `false`, disables the RSS processor | `true`  |

#### Feed Types

Available feed type options:

- `AI`
- `DATA`
- `DEVOPS`
- `TECH`

### Database

The database is required for the RSS processor to store feed items.

#### Database Configuration

| Field      | Description       | Example     |
|------------|-------------------|-------------|
| `host`     | Server IP address | `127.0.0.1` |
| `port`     | Server port       | `5432`      |
| `user`     | Database user     | `root`      |
| `password` | Database password | `passwd`    |
| `database` | Database name     | `example`   |

### AI

The AI summary feature is optional. It allows you to create summaries from different emails, store them, and send them
to applications like Discord.

**Global configuration**:

All prompts are located in the `config/ai` directory.

| Field                                | Description                                                   | Default                               |
|--------------------------------------|---------------------------------------------------------------|---------------------------------------|
| `enable`                             | If `false`, the AI flow is disabled                           | `true`                                |
| `article_summary_system_prompt_path` | System prompt to create a summary for a single email          | `ai_article_summary_system_prompt.md` |
| `global_summary_system_prompt_path`  | System prompt for the global summary from all individual ones | `ai_global_summary_system_prompt.md`  |
| `user_prompt_path`                   | User message sent with the file                               | `ai_user_message.md`                  |

#### Using Gemini

This crate uses `gemini-rust`. You must configure an API key on [Google AI Studio](https://aistudio.google.com/). To
configure it, fill in the `[ai.gemini]` section in your configuration file. For the `model` field, the complete list of
Gemini models is available [here](https://ai.google.dev/gemini-api/docs/models).

#### Using Mistral Vibe

*Mistral Vibe configuration details coming soon.*

### Notification

#### Using Discord

First, you need to [create a webhook](https://support.discord.com/hc/en-us/articles/228383668-Intro-to-Webhooks) on the
Discord channel where you want to send the daily summary. Then, fill in the `[notification.discord]` section in your
configuration file and paste the webhook URL in the `webhook_url` field.

## Configuration

| Field             | Description                                                                        | Default                   |
|-------------------|------------------------------------------------------------------------------------|---------------------------|
| `mode`            | Current environment. Create a `<mode>.toml` file to override default configuration | `dev`                     |
| `repository_path` | Path where the files will be stored                                                | `tldr_newsletter_storage` |