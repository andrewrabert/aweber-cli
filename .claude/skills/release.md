---
name: release
description: This skill should be used when the user asks to "cut a release", "tag a release", "prepare a release", "bump the version", or mentions releasing a new version. Guides the full release process from changelog through tagging.
---

# Release Process

Follow these steps in order when preparing a new release.

## 1. Determine the Version

Check the current version in `Cargo.toml` and the git tags. The new version must follow [Semantic Versioning](https://semver.org/):

- **patch** (0.1.0 -> 0.1.1): bug fixes, dependency updates, no API changes
- **minor** (0.1.0 -> 0.2.0): new features, backwards-compatible API additions
- **major** (0.1.0 -> 1.0.0): breaking changes

If the user hasn't specified which bump, recommend one based on the changes since the last tag and confirm before proceeding.

## 2. Update CHANGELOG.md

Use the **changelog** skill for format and style guidance. Move entries from `[Unreleased]` into a new version heading, or write new entries by inspecting `git log <last-tag>..HEAD`.

## 3. Bump Version in Cargo.toml

Update the `version` field in `Cargo.toml`. Run `cargo check` to ensure `Cargo.lock` updates cleanly.

## 4. Commit

Create a single commit with message: `Release v<version>`

Stage only `CHANGELOG.md`, `Cargo.toml`, and `Cargo.lock` (if changed).

## 5. Tag

Create an annotated tag:

```sh
git tag -a v<version> -m "v<version>"
```

## 6. Push

Push both the commit and the tag. The release workflow (`.github/workflows/release.yml`) triggers on `v*` tags and builds binaries for all targets automatically.

```sh
git push && git push --tags
```

Always confirm with the user before pushing.
