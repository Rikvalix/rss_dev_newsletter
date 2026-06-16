# Tldr Newsletter

Fetch, Store emails from the [TLDR newsletter](https://tldr.tech/newsletters), generate resume and send it on discord
thanks to AI.

<!-- TOC -->
* [Tldr Newsletter](#tldr-newsletter)
* [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Installation](#installation)
    * [Email](#email)
      * [Using Gmail](#using-gmail)
    * [AI](#ai)
      * [Using Gemini](#using-gemini)
  * [Configuration](#configuration)
<!-- TOC -->

My first Rust project, currently in development. Right now, only the fetch and storage features work, and all values are
hardcoded (except for credentials).

# Getting Started

## Prerequisites

- Rust on your environment, [install it](https://rust-lang.org/fr/tools/install/)

## Installation

Rename the configuration file from `default-template.toml` to `default.toml`

### Email

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

#### Using Gemini

## Configuration

| Field                | Description                                                                        | Default                   |
|----------------------|------------------------------------------------------------------------------------|---------------------------|
| `mode`               | Current environment. Create a `<mode>.toml` file to override default configuration | `dev`                     |
| **tldr_settings**    | ---------                                                                          | ------                    |
| `mark_email_as_read` | If `false`, every run will pull your entire inbox                                  | `false`                   |
| **git_settings**     | --------                                                                           | ---                       |
| `enable`             | If `true`, the content of your `repository_path` will be pushed to the `branch`    | `false`                   |
| `branch`             | Default branch                                                                     | `main`                    |
| **storage_settings** | ----                                                                               | ---                       |
| `repository_path`    | Path of the repository where the files will be stored                              | `tldr_newsletter_storage` |