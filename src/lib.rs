//! # mcp-translate
//!
//! Translation MCP server — translate text, detect languages, batch translate,
//! and search translation memory across 200+ languages.
//!
//! ## Quick Start
//!
//! ```bash
//! cargo install mcp-translate
//! mcp-translate
//! ```
//!
//! Zero configuration required. No API keys needed.
//!
//! ## Tools
//!
//! | Tool | Description |
//! |------|-------------|
//! | `translate` | Translate text between any two languages |
//! | `detect_language` | Detect the language of input text |
//! | `batch_translate` | Translate into multiple target languages at once |
//! | `search_translation_memory` | Search human-verified translations |
//! | `list_languages` | List supported languages with ISO 639-1 codes |
//!
//! ## Backend
//!
//! Uses [MyMemory](https://mymemory.translated.net/) — free, 200+ languages,
//! includes human-verified translation memory from EU/UN corpora.

pub mod server;
