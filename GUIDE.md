# AWeber CLI user guide

Run `aweber <group> --help` for actions and `aweber <group> <action> --help`
for exact flags. This guide explains how to combine them.

## guide

`aweber guide` prints this document. Use `aweber guide --list` for section names
or `aweber guide workflows` for one section. Section names are case-insensitive.
These commands work offline before you log in.

## auth

Run `aweber auth login` to authorize in your browser, then paste the returned
code. `aweber auth status` shows the current session. `aweber auth logout`
removes the stored credentials. Use `--credentials-file` to select a separate
session file or `AWEBER_TOKEN` to supply a token for a script.

## output and errors

- Collection actions such as `lists list` print newline-delimited JSON when
  redirected or piped. Most collection actions fetch every page. Apply `--limit`
  when an action offers it to cap the total number of entries. Some list actions
  have their own pagination flags. Check the action's help.
- An action such as `lists get` or `workflows show` prints one JSON document.
  Some document outputs are arrays, such as `messages get`. Some successful
  mutations, such as `subscribers unsubscribe`, have no stdout output.
- Exit 0 means success, 1 means the action failed, and 2 means invalid usage.
  Read stderr for diagnostics even if an action produced partial stdout output.
  `api --json` also reports the HTTP status and body in its stdout document.

## account

`account list` shows available accounts. `account get` uses the authenticated
account. Use the account actions to find across lists or inspect account-level
web forms when you do not know which list owns a resource.

## api

`aweber api /1.0/accounts` sends an authenticated GET and writes the response
body to stdout and response headers to stderr. `--json` wraps status, headers,
and body in a single document. `--method` changes the method; `--input -` reads
the request body from stdin. Review the method and path before sending a request
that changes account data.

## lists

Start with `aweber lists list --limit 10`. Many commands accept either numeric
`--list-id` or `--list` with a list name or unique ID (an `awlist…` value).
`--list` looks up the list. Use the numeric ID when an action accepts it and you
already know it. Workflow actions that take `--list` use the name or unique ID.

## subscribers

`subscribers get` accepts a numeric subscriber ID or an email on the selected
list. Use `subscribers find` to look for matches. `subscribers unsubscribe`
keeps the subscriber's history. `subscribers delete` deletes the record; check
the list and subscriber before running it.

## broadcasts

1. Draft with `broadcasts create --list-id LIST_ID --subject SUBJECT --body-text TEXT --body-html HTML`. Read the numeric ID from its result.
2. Use `broadcasts get --list-id LIST_ID --broadcast-id BROADCAST_ID` to review the draft and recipients. Set a date only after confirming the content and intended list.
3. `broadcasts schedule --list-id LIST_ID --broadcast-id BROADCAST_ID --scheduled-for TIME` schedules an actual email send. `TIME` is an ISO 8601 timestamp such as `2030-01-01T12:00:00Z`.
4. `broadcasts wait --list-id LIST_ID --broadcast-id BROADCAST_ID` polls for completion. The default interval is 30 seconds.

Ordinary broadcast actions take a numeric `--broadcast-id`. `link-analytics`
uses the broadcast UUID. Deleting a draft and canceling a scheduled send are
separate actions.

## campaigns

Use `campaigns list` for follow-up or broadcast messages on a list, with
`campaigns stats` for collection statistics. The `workflows` group is for
editing automation workflows.

## custom-fields

Inspect a list with `custom-fields list` before creating or updating a field.
Deleting a custom field can remove subscriber data stored in it.

## integrations

Use `integrations list` to find integrations and `integrations get` to inspect
one. Where a broadcast asks for an integration URL, use its resource link.

## landing-pages

Use `landing-pages list` on a list to find its landing pages and
`landing-pages get` to inspect one.

## messages

Message IDs are 24 hexadecimal characters. `messages list` shows unbound drafts
by default; use its `--binding` option for other bindings. `messages get` takes
one or more IDs. `messages create --list-id LIST_ID --subject SUBJECT --body-text TEXT`
creates an unbound draft. Workflow message steps refer to this ID.
`messages delete` removes an unbound message and its drafts and attachments.

## purchases

`purchases create` records a purchase for a subscriber on a list. It changes
account data; inspect its required fields and confirm the subscriber first.

## segments

`segments list` finds a list's segments. `segments get` takes the segment ID.

## tags

`tags list` displays known tags for a list. Workflow starters and tag steps use
tag names, and subscriber actions can add and remove those names.

## webform-split-tests

Use `webform-split-tests list` to discover tests and `webform-split-tests get`
to inspect one. The `components` and `component` actions inspect the
forms in a test.

## webforms

`webforms list` shows web forms on a list. `webforms get` inspects a specific
form. For an account-wide view, use the account group.

## workflows

1. Create an unbound draft with `messages create` and save its message ID.
2. Run `workflows create NAME --list LIST` and save the workflow UUID. A workflow lookup also accepts a name; `--list` can disambiguate names.
3. Add a message step with `workflows add-step WORKFLOW_ID message MESSAGE_ID`. Use `workflows add-step --help` for wait, tag, feed and split steps, or `workflows update-step` to edit a step by its ID.
4. `workflows show WORKFLOW_ID` shows the working copy. Use `--draft` for only unpublished changes and `--published` to see the published version.
5. `workflows publish WORKFLOW_ID` publishes the working copy. It changes what runs for subscribers on an active workflow, so check the starter, steps and messages before publishing. `workflows publish WORKFLOW_ID --discard` discards unpublished changes instead.

`workflows delete` removes the workflow and returns its messages to Drafts.
