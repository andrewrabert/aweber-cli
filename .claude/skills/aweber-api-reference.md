---
name: aweber-api-reference
description: Use when fetching, reading, or referencing the AWeber API spec/docs, working with AWeber API endpoints, adding or modifying API calls, checking parameter names, response schemas, or verifying endpoint paths and HTTP methods
---

# AWeber API Reference

## Canonical Sources

- **Interactive docs:** https://api.aweber.com/
- **OpenAPI spec:** https://api.aweber.com/swagger.yaml

Always consult these before guessing API behavior.

## Fetching the Full Spec

The root swagger.yaml uses `$ref` externals and is not self-contained. Bundle it first:

```sh
npx @redocly/cli@latest bundle https://api.aweber.com/swagger.yaml -o /tmp/aweber-api-bundled.yaml
```

Then read the bundled file to look up endpoints, parameters, types, and schemas.
