# Changelog

## [1.1.0] - 2026-08-13

### Changed
- Upgraded to rmcp 3.1.2 and raised the minimum supported Rust version to 1.94.1.
- Added MCP 2026-07-28 stateless request handling while retaining MCP 2025-11-25 initialization compatibility.

### Added
- Per-request identity and protocol metadata, on-demand discovery/cache hints, and the configured Tasks and sealed MRTR approval policies.

All notable changes to this project will be documented in this file.

## [1.0.0] - 2026-05-27

### Added
- `translate` — translate text between 200+ languages with quality scoring
- `detect_language` — automatic language detection
- `batch_translate` — translate into multiple target languages in one call
- `search_translation_memory` — search human-verified translations from MyMemory
- `list_languages` — list supported languages with ISO 639-1 codes
- MyMemory API backend (zero config, no API keys)
- Autodetect source language support (`source: "auto"`)
- Quality scoring on all translations
- Alternative translations from translation memory
- ADK-Rust Enterprise registry manifest (`mcp-server.toml`)
- Architecture documentation and SVG diagram
