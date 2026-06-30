# Tldr Newsletter

Fetch, Store emails from the [TLDR newsletter](https://tldr.tech/newsletters), generate summary and send it on discord
thanks to AI. This is my first Rust project, currently in development.

<!-- TOC -->
* [Tldr Newsletter](#tldr-newsletter)
* [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Installation](#installation)
    * [Email](#email)
      * [Using Gmail](#using-gmail)
    * [AI](#ai)
      * [Using Gemini](#using-gemini)
      * [Using Mistral Vibe](#using-mistral-vibe)
    * [Notification](#notification)
      * [Using Discord](#using-discord)
  * [Configuration](#configuration)
<!-- TOC -->

# Getting Started

## Prerequisites

- Rust on your environment, [install it](https://rust-lang.org/fr/tools/install/)

## Installation

Rename the configuration file from `default-template.toml` to `default.toml`

### Email

**Global configuration**:

| Field                | Description                                       | Default |
|----------------------|---------------------------------------------------|---------|
| `mark_email_as_read` | If `false`, every run will pull your entire inbox | `false` |

#### Using Gmail

_I recommend creating a dedicated Gmail address because TLDR sends around 5 emails per day._

To use Gmail, you need to follow these steps:

- Create an application on the [Google Cloud Console](https://console.cloud.google.com/)
- Enable
  the [Gmail API](https://console.cloud.google.com/apis/library/gmail.googleapis.com?organizationId=0&supportedpurview=project)
  for your project
- Configure the OAuth 2 settings for your application
- Store the secret in `client_secret.json`
- The first run will log the application into your Google account; this will create a `token.json` file
- Re-run and enjoy!

### AI

The AI summary is an optional flow. It allows you to create summaries from different emails, store them, and send them
to your favorite application like Discord.

**Global configuration**:

All prompts are located in the `config/ai` directory.

| Field                                | Description                                                    | Default                               |
|--------------------------------------|----------------------------------------------------------------|---------------------------------------|
| `enable`                             | if `false` the ai flow is disable.                             | `true`                                |
| `article_summary_system_prompt_path` | System prompt to create a summary for a single email.          | `ai_article_summary_system_prompt.md` |
| `global_summary_system_prompt_path`  | System prompt for the global summary from all individual ones. | `ai_global_summary_system_prompt.md`  |
| `user_prompt_path`                   | User message send with the file.                               | `ai_user_message.md`                  |

#### Using Gemini

The crate use is `gemini-rust`, you must configure API key on [google ai-studio](https://aistudio.google.com/). To
configure it just fill the `[ai.gemini]` section. For the `model` field the complete list of Gemini models is
available [here](https://ai.google.dev/gemini-api/docs/models).

#### Using Mistral Vibe

### Notification

#### Using Discord

First you need [create webhook](https://support.discord.com/hc/en-us/articles/228383668-Intro-to-Webhooks) on the
discord channel where you want to send the daily summary. After that fill the `[notification.discord]` section and paste
the webhook url in the field `webhook_url`.

## Configuration

| Field                | Description                                                                        | Default                   |
|----------------------|------------------------------------------------------------------------------------|---------------------------|
| `mode`               | Current environment. Create a `<mode>.toml` file to override default configuration | `dev`                     |
| **storage_settings** | ----                                                                               | ---                       |
| `repository_path`    | Path of the repository where the files will be stored                              | `tldr_newsletter_storage` |