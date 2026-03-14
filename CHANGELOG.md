# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2026-03-13

### Added
- Initial release
- Diff two `.env` files with secrets masked by default (`****tail`)
- Bot mode (`--bot`): clean JSON output, no ANSI, designed for AI agents
- Machine-readable interface spec (`--bot-help`, ~200 tokens)
- `--all` flag: include matching keys in output
- `--reveal` flag: opt-in full value display (dangerous)
- `-o <file>`: write JSON output to file
- Handles: quotes (single/double), `export` prefix, multiline values, BOM, CRLF, inline comments, duplicate keys (last wins)
- Exit codes: 0 (match), 1 (differences), 2 (error)
- MIT license
