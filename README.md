# aweber-cli

[![Crates.io](https://img.shields.io/crates/v/aweber)](https://crates.io/crates/aweber)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/andrewrabert/aweber-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/andrewrabert/aweber-cli/actions/workflows/ci.yml)

A command-line interface and Rust client library for the [AWeber API](https://api.aweber.com/).

⚠️ Under active development

## Download

Pre-built binaries for Linux (glibc & musl), macOS, and Windows are available on the [latest release](https://github.com/andrewrabert/aweber-cli/releases/latest) page.

## Usage

```sh
$ aweber --help
AWeber API CLI

Usage: aweber [OPTIONS] <COMMAND>

Commands:
  auth                 Manage authentication
  lists                Manage subscriber lists
  subscribers          Manage subscribers
  broadcasts           Manage broadcasts (email campaigns)
  campaigns            Manage campaigns
  account              Manage your AWeber account
  custom-fields        Manage custom fields
  tags                 Manage tags
  segments             Manage segments
  integrations         Manage integrations
  landing-pages        Manage landing pages
  purchases            Record purchases
  webforms             Manage webforms
  webform-split-tests  Manage webform split tests
  help                 Print this message or the help of the given subcommand(s)

Options:
      --base-url <base-url>  AWeber API base URL [env: AWEBER_API_URL=] [default: https://api.aweber.com/1.0]
      --token <token>        OAuth2 access token (overrides stored credentials) [env: AWEBER_TOKEN=]
  -h, --help                 Print help
```

### Authentication

Log in via OAuth2 (opens a browser):

```sh
aweber auth login
```

Check auth status:

```sh
aweber auth status
```

### Examples

List all subscriber lists:

```
$ aweber lists list
{
  "id": 1,
  "name": "Curated Tech News",
  "total_subscribed_subscribers": 0,
  "total_subscribers": 205,
  "total_subscribers_subscribed_today": 4,
  "total_subscribers_subscribed_yesterday": 0,
  "total_unconfirmed_subscribers": 3,
  "total_unsubscribed_subscribers": 0,
  "unique_list_id": "awlist1",
  "uuid": "e7a69cec-851e-4418-acd5-04ccc312c91c"
}
{
  "id": 2,
  "name": "Personal Newsletter",
  "total_subscribed_subscribers": 1,
  "total_subscribers": 40,
  "total_subscribers_subscribed_today": 1,
  "total_subscribers_subscribed_yesterday": 0,
  "total_unconfirmed_subscribers": 0,
  "total_unsubscribed_subscribers": 0,
  "unique_list_id": "awlist2",
  "uuid": "9ef78f93-648b-48db-898f-54a090fc5a58"
}
```

Get a subscriber by list name and email:

```
$ aweber subscribers get --list 'Curated Tech News' --email user@example.com
{
  "custom_fields": {
    "a": "b",
    "z": "y"
  },
  "email": "user@example.com",
  "id": 789,
  "is_verified": true,
  "last_followup_message_number_sent": 1001,
  "status": "subscribed",
  "subscribed_at": "2026-03-12T23:42:55.877144+00:00",
  "subscription_method": "api",
  "uuid": "4a743845-8d6e-4876-9099-87d3d0bcb899",
  "verified_at": "2026-03-12T19:43:16+00:00"
}
```
