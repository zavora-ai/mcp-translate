# Translation MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-translate.svg)](https://crates.io/crates/mcp-translate)
[![Docs.rs](https://docs.rs/mcp-translate/badge.svg)](https://docs.rs/mcp-translate)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)
[![Registry Ready](https://img.shields.io/badge/ADK_Registry-Ready-green.svg)](https://enterprise.adk-rust.com)

Multilingual translation for [ADK-Rust Enterprise](https://enterprise.adk-rust.com) agents. Provides 5 MCP tools across 200+ languages — **zero configuration, no API keys required**.

## Architecture

<p align="center">
  <img src="https://raw.githubusercontent.com/zavora-ai/mcp-translate/main/docs/assets/architecture.svg" alt="Translation MCP Architecture" width="800"/>
</p>

## Key Principles

- **Zero configuration** — works out of the box with no API keys or environment variables.
- **200+ languages** — covers every major language including African, Asian, and indigenous languages.
- **Translation memory** — leverages millions of human-verified translations from MyMemory.
- **Quality scoring** — every translation includes a confidence/quality score.
- **Batch support** — translate into multiple languages in a single call.
- **Registry-ready** — ships with `mcp-server.toml` for automatic ADK-Rust Enterprise onboarding.

## Tools

| Tool | Purpose | Risk Class |
|------|---------|------------|
| `translate` | Translate text between any two languages | Read-only |
| `detect_language` | Detect the language of input text | Read-only |
| `batch_translate` | Translate text into multiple target languages at once | Read-only |
| `search_translation_memory` | Search human-verified translation memory | Read-only |
| `list_languages` | List supported languages with ISO 639-1 codes | Read-only |

## Backends

| Backend | Purpose | Rate Limit |
|---------|---------|-----------|
| MyMemory | Primary translation engine | 5,000 words/day (anonymous) |
| MyMemory (with email) | Extended quota | 30,000 words/day |

## Installation

### From crates.io

```bash
cargo install mcp-translate
```

### Build from source

```bash
git clone https://github.com/zavora-ai/mcp-translate
cd mcp-translate
cargo build --release
```

The binary is at `target/release/mcp-translate`.

### Claude Desktop

Add to `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS) or `%APPDATA%\Claude\claude_desktop_config.json` (Windows):

```json
{
  "mcpServers": {
    "translate": {
      "command": "mcp-translate"
    }
  }
}
```

### Kiro

Add to your project's `.kiro/settings/mcp.json`:

```json
{
  "mcpServers": {
    "translate": {
      "command": "mcp-translate"
    }
  }
}
```

### Cursor

Add to `.cursor/mcp.json` in your project root:

```json
{
  "mcpServers": {
    "translate": {
      "command": "mcp-translate"
    }
  }
}
```

### Windsurf

Add to `~/.codeium/windsurf/mcp_config.json`:

```json
{
  "mcpServers": {
    "translate": {
      "command": "mcp-translate"
    }
  }
}
```

### Codex (OpenAI)

Add to `~/.codex/config.json`:

```json
{
  "mcpServers": {
    "translate": {
      "command": "mcp-translate"
    }
  }
}
```

### Open Code

Add to `~/.config/opencode/config.json`:

```json
{
  "mcp": {
    "translate": {
      "command": "mcp-translate"
    }
  }
}
```

### Any MCP Client (Streamable HTTP)

```bash
mcp-translate --transport http --port 8080
```

Then connect your client to `http://localhost:8080/mcp`.

### Docker

```bash
docker run -p 8080:8080 ghcr.io/zavora-ai/mcp-translate:latest
```

## Quick Start

### Translate text

```json
{
  "name": "translate",
  "arguments": {
    "text": "The future of artificial intelligence is bright",
    "source": "en",
    "target": "sw"
  }
}
```

**Response:**

```json
{
  "translation": "Mustakabali wa akili bandia ni mzuri",
  "source_lang": "en",
  "target_lang": "sw",
  "quality": 0.85,
  "alternatives": [
    {
      "translation": "Mustakabali wa akili bandia ni mzuri",
      "quality": "85",
      "source": "Machine Translation"
    }
  ]
}
```

### Detect language

```json
{
  "name": "detect_language",
  "arguments": {
    "text": "Bonjour le monde, comment allez-vous?"
  }
}
```

**Response:**

```json
{
  "detected_language": "fr",
  "text_sample": "Bonjour le monde, comment allez-vous?"
}
```

### Batch translate

```json
{
  "name": "batch_translate",
  "arguments": {
    "text": "Hello world",
    "source": "en",
    "targets": ["fr", "de", "ja", "ar", "sw"]
  }
}
```

**Response:**

```json
{
  "source": "en",
  "original": "Hello world",
  "translations": [
    { "language": "fr", "translation": "Bonjour le monde" },
    { "language": "de", "translation": "Hallo Welt" },
    { "language": "ja", "translation": "こんにちは、世界。" },
    { "language": "ar", "translation": "مرحبا بكم" },
    { "language": "sw", "translation": "Jambo dunia!" }
  ]
}
```

### Search translation memory

```json
{
  "name": "search_translation_memory",
  "arguments": {
    "text": "Terms and conditions",
    "source": "en",
    "target": "fr"
  }
}
```

**Response:**

```json
{
  "query": "Terms and conditions",
  "langpair": "en|fr",
  "matches": 5,
  "results": [
    {
      "segment": "Terms and Conditions",
      "translation": "Termes et conditions",
      "quality": "100",
      "source": "European Commission",
      "last_updated": "2024-03-15"
    }
  ]
}
```

## Configuration

### Environment Variables

| Variable | Required | Purpose |
|----------|:--------:|---------|
| `MYMEMORY_EMAIL` | No | Increases daily quota from 5,000 to 30,000 words |
| `RUST_LOG` | No | Log level (default: `info`) |

### MCP Server Manifest

```toml
server_id = "mcp_translate"
display_name = "Translate"
version = "1.0.0"
domain = "translation"
risk_level = "low"
writes_allowed = "none"
transports = ["stdio"]
governance_gates = []
```

## Supported Languages

### Major Languages

| Code | Language | Code | Language | Code | Language |
|:----:|----------|:----:|----------|:----:|----------|
| `en` | English | `fr` | French | `de` | German |
| `es` | Spanish | `pt` | Portuguese | `it` | Italian |
| `nl` | Dutch | `ru` | Russian | `zh` | Chinese |
| `ja` | Japanese | `ko` | Korean | `ar` | Arabic |
| `hi` | Hindi | `bn` | Bengali | `ur` | Urdu |
| `tr` | Turkish | `pl` | Polish | `uk` | Ukrainian |
| `vi` | Vietnamese | `th` | Thai | `id` | Indonesian |

### African Languages

| Code | Language | Code | Language | Code | Language |
|:----:|----------|:----:|----------|:----:|----------|
| `sw` | Swahili | `am` | Amharic | `ha` | Hausa |
| `yo` | Yoruba | `ig` | Igbo | `zu` | Zulu |
| `af` | Afrikaans | `rw` | Kinyarwanda | `so` | Somali |

### Nordic Languages

| Code | Language | Code | Language | Code | Language |
|:----:|----------|:----:|----------|:----:|----------|
| `sv` | Swedish | `no` | Norwegian | `da` | Danish |
| `fi` | Finnish | `et` | Estonian | `lv` | Latvian |

### Special Codes

| Code | Meaning |
|:----:|---------|
| `auto` | Autodetect source language |

Full list of 200+ languages available via the `list_languages` tool or at [mymemory.translated.net](https://mymemory.translated.net/doc/languages.php).

## Use Cases

### Multilingual Customer Support
```
Agent receives message in unknown language
→ detect_language → "sw" (Swahili)
→ translate (sw→en) → English for agent processing
→ translate (en→sw) → Reply in customer's language
```

### Document Localization
```
Agent has English documentation
→ batch_translate (en → [fr, de, es, ja, zh])
→ 5 localized versions in one call
```

### Legal/Compliance Translation
```
Agent needs official translation reference
→ search_translation_memory ("Terms and conditions", en→fr)
→ Returns EU Commission verified translation (quality: 100%)
```

### Content Quality Assurance
```
Agent receives translation from external source
→ translate (same text, same pair)
→ Compare quality scores and alternatives
→ Flag discrepancies for human review
```

## Testing

```bash
# Build
cargo build

# Run (zero config)
cargo run

# Test with MCP client
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0.1"}}}' | cargo run 2>/dev/null
```

## Documentation

| Document | Description |
|----------|-------------|
| [mcp-server.toml](mcp-server.toml) | ADK-Rust Enterprise registry manifest |
| [CHANGELOG.md](CHANGELOG.md) | Version history |
| [Rust Docs](https://docs.rs/mcp-translate) | Generated API documentation |

## Contributing

Contributions welcome. Priority areas:
- Additional translation backends (DeepL, Google Translate with API key)
- Glossary/terminology management
- Translation caching layer
- Document-level translation with formatting preservation

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START -->
| [<img src="https://github.com/jkmaina.png" width="80px;" alt=""/><br /><sub><b>James Karanja Maina</b></sub>](https://github.com/jkmaina) |
|:---:|
<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Apache-2.0 — see [LICENSE](LICENSE) for details.

---

Part of the [ADK-Rust Enterprise](https://enterprise.adk-rust.com) MCP server ecosystem.

Built with ❤️ by [Zavora AI](https://zavora.ai)

## Registry Compliance

This server implements the [ADK MCP SDK](https://crates.io/crates/adk-mcp-sdk) contract:

- **HealthCheck** — async health probe for registry monitoring
- **mcp-server.toml** — manifest declaring tools, risk classes, and credentials
- **Structured tracing** — `RUST_LOG` env-filter for observability

## rmcp and MCP compatibility

This server is built with [`rmcp` 3.1.2](https://github.com/modelcontextprotocol/rust-sdk/releases/tag/rmcp-v3.1.2) and requires Rust 1.94.1 or newer. The rmcp 3 rollout retains legacy MCP initialization compatibility and targets MCP protocol revisions `2025-11-25` and `2026-07-28`.

## MCP 2026-07-28 rollout (P5 read-heavy)

This server uses `rmcp` 3.1.2 and `adk-mcp-sdk` 0.2 with a minimum supported
Rust version of **1.94.1**. It accepts stateless MCP 2026 requests with
per-request protocol, client identity, and capability metadata while retaining
the legacy MCP 2025-11-25 initialize flow for ordinary tools.

- **Tasks:** `batch_translate`
- **MRTR approvals:** None; this server exposes no manifest-classified protected operations.
- **Discovery and routing:** rmcp serves on-demand discovery and validates the
  per-request protocol envelope; HTTP deployments can route with `Mcp-Method`
  and `Mcp-Name`. The packaged binary currently uses stdio.
- **Caching:** `tools/list` returns a public `ttlMs` of 60,000 for MCP 2026;
  rmcp omits the cache fields for legacy clients.
- **Deprecated extensions:** this server does not add new Roots, Sampling, or
  dynamic client-registration dependencies.

Protected tools require `MCP_REQUEST_STATE_KEY` with at least 32 high-entropy
bytes. All replicas must share that key so sealed approval state can resume on
another instance. Approval state is bound to the client identity, tool, and
arguments and expires after two minutes. Missing identity, invalid state,
rejection, or legacy protocol use fails closed. Task records are process-local
for the current stdio runtime; use a durable task store before deploying the
server behind scale-to-zero HTTP infrastructure.
