# sam-env-diff

Diff your .env files. Secrets stay masked. Written in Rust.

```bash
sam-env-diff .env .env.example
```

That's it.

## Bot Mode (Primary Interface)

Designed for AI agents. Clean JSON, no ANSI, no chrome.

```bash
sam-env-diff .env .env.example --bot
```

```json
{
  "left": ".env",
  "right": ".env.example",
  "missing": [{"key": "DATABASE_URL"}],
  "extra": [{"key": "DEBUG_MODE", "val": "****true"}],
  "changed": [{"key": "API_KEY", "left": "****7f3a", "right": "****0000"}],
  "match": 12,
  "ok": false
}
```

### Machine-Readable Interface Spec

```bash
sam-env-diff --bot-help
```

One call, ~200 tokens. A bot knows the full interface without parsing docs.

## All Flags

```bash
sam-env-diff .env .env.example          # 90% use case
sam-env-diff .env .env.example --all    # show matching keys too
sam-env-diff .env .env.example --bot    # JSON output
sam-env-diff .env .env.example -o out.json  # write JSON to file
sam-env-diff .env .env.example --reveal # unmask values (dangerous!)
sam-env-diff --bot-help                 # machine-readable spec
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0    | Files match |
| 1    | Differences found |
| 2    | Error (bad args, file not found) |

## Why This Exists

Every `.env` comparison tool either shows full secrets in the terminal or
doesn't understand the format. This one masks by default (`****7f3a`), handles
every edge case (quotes, export prefix, multiline, BOM, CRLF), and speaks JSON
for bots.

## Performance

Single binary, no runtime, instant startup. Written in Rust.

```
time sam-env-diff large.env large2.env --bot
```

## Install

```bash
cargo install --path .
```

## License

MIT — Sam M., 2026
