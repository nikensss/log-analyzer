# bf-log-errors

Scans a "logentries" file for interesting errors.

To run it, download a collection of logs from "logentries", unzip it and name
it "log", and move it to the root of this repo. Then, run `cargo run --release`
and, once done, you will have three files available: `by_id.json`,
`unique_errors.json` and `unknown_errors.json`.

## `by_id.json`

Shows the log messages grouped by `reqId`.

## `unique_errors.json`

After grouping the requests by `reqId`, pattern matching is done on each line
of each group trying to find what the error actually was. For each match a
count is kept to know how often it happened. If a request brings no match, it
is sent to the `unknown_errors.json` file.

## `unknown_errors.json`

Shows all the requests for which pattern matching was not successful. This
program follows an optimistic approach on the pattern matching, assuming that
almost all requests will successfully match, and the few ones that don't can be
scanned visually and quickly.
