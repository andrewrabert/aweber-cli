---
name: changelog
description: This skill should be used when the user asks to "update the changelog", "add a changelog entry", "write changelog for a release", "prepare a release", or mentions CHANGELOG.md. Covers format, audience sections, and writing style for this project.
---

# Changelog Maintenance

This project follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format with [Semantic Versioning](https://semver.org/).

## Format

The file is `CHANGELOG.md` in the project root. Releases are sorted newest-first. Each release uses a second-level heading with the version (no "v" prefix) and date in ISO 8601:

```markdown
## [0.2.0] - 2026-04-01
```

An `[Unreleased]` section at the top collects changes that haven't been tagged yet.

## Change Categories

Use only the categories that have entries. Order them as listed:

- **Added** - new features or capabilities
- **Changed** - changes to existing functionality
- **Deprecated** - features that will be removed in a future release
- **Removed** - features that were removed
- **Fixed** - bug fixes
- **Security** - vulnerability fixes

## Audience Sections

This project is both a CLI tool and a Rust library. Within each change category, group entries under subsections when both audiences are affected:

```markdown
### Added

#### CLI
- New `schedule-broadcast` command

#### Library
- `Client::schedule_broadcast` method
```

When a change only affects one audience or applies to both equally (e.g. dependency updates, build fixes), skip the subsections and list entries directly under the category.

## Writing Style

- Write entries in imperative mood: "Add feature" not "Added feature" or "Adds feature"
- Keep entries concise - one line per change
- Focus on what changed from the user's perspective, not implementation details
- Link to PRs or issues when relevant: `([#1](url))`
- Prefix breaking changes with **BREAKING:**
- Do not include changes that are purely internal (CI tweaks, dev tooling) unless they affect how users build or install the project

## Determining Changes

To determine what changed between releases, inspect the git log between the previous tag (or initial commit) and HEAD:

```sh
git log --oneline <previous-tag>..HEAD
```

Translate commit messages into user-facing changelog entries. Multiple related commits may collapse into a single entry. Not every commit warrants an entry - skip internal-only changes.
