use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use reqwest::Client;
use serde_json::{json, Value};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct TranslateInput {
    /// Text to translate
    pub text: String,
    /// Source language code (e.g. "en", "fr", "auto" for autodetect)
    pub source: String,
    /// Target language code (e.g. "es", "de", "sw")
    pub target: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct DetectInput {
    /// Text to detect language of
    pub text: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BatchInput {
    /// Text to translate
    pub text: String,
    /// Source language code
    pub source: String,
    /// Target language codes (e.g. ["fr", "de", "es", "sw"])
    pub targets: Vec<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct MemoryInput {
    /// Text to search in translation memory
    pub text: String,
    /// Source language code
    pub source: String,
    /// Target language code
    pub target: String,
}

#[derive(Clone)]
pub struct TranslateServer {
    pub client: Client,
}

impl TranslateServer {
    async fn mymemory(&self, text: &str, source: &str, target: &str) -> Result<Value, String> {
        let src = if source == "auto" { "autodetect" } else { source };
        let url = format!(
            "https://api.mymemory.translated.net/get?q={}&langpair={}|{}",
            urlencoding::encode(text), src, target
        );
        let resp = self.client.get(&url).send().await.map_err(|e| e.to_string())?;
        resp.json::<Value>().await.map_err(|e| e.to_string())
    }
}

#[tool_router]
impl TranslateServer {
    #[tool(description = "Translate text between languages. Use source='auto' for autodetect. Supports 200+ languages.")]
    async fn translate(&self, Parameters(input): Parameters<TranslateInput>) -> String {
        match self.mymemory(&input.text, &input.source, &input.target).await {
            Ok(data) => {
                let translated = data["responseData"]["translatedText"].as_str().unwrap_or("");
                let quality = data["responseData"]["match"].as_f64().unwrap_or(0.0);
                let matches: Vec<Value> = data["matches"].as_array().unwrap_or(&vec![]).iter().take(3).map(|m| json!({
                    "translation": m["translation"], "quality": m["quality"], "source": m["created-by"]
                })).collect();
                json!({
                    "translation": translated, "source_lang": input.source, "target_lang": input.target,
                    "quality": quality, "alternatives": matches
                }).to_string()
            }
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Detect the language of input text")]
    async fn detect_language(&self, Parameters(input): Parameters<DetectInput>) -> String {
        match self.mymemory(&input.text, "autodetect", "en").await {
            Ok(data) => {
                let detected = data["responseData"]["detectedLanguage"].as_str()
                    .or_else(|| data["matches"].as_array().and_then(|m| m.first()).and_then(|m| m["source"].as_str()))
                    .unwrap_or("unknown");
                json!({"detected_language": detected, "text_sample": &input.text[..input.text.len().min(50)]}).to_string()
            }
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Translate text into multiple target languages at once")]
    async fn batch_translate(&self, Parameters(input): Parameters<BatchInput>) -> String {
        let mut results = Vec::new();
        for target in &input.targets {
            match self.mymemory(&input.text, &input.source, target).await {
                Ok(data) => {
                    let translated = data["responseData"]["translatedText"].as_str().unwrap_or("").to_string();
                    results.push(json!({"language": target, "translation": translated}));
                }
                Err(e) => results.push(json!({"language": target, "error": e})),
            }
        }
        json!({"source": input.source, "original": input.text, "translations": results}).to_string()
    }

    #[tool(description = "Search MyMemory translation memory for existing human-verified translations")]
    async fn search_translation_memory(&self, Parameters(input): Parameters<MemoryInput>) -> String {
        match self.mymemory(&input.text, &input.source, &input.target).await {
            Ok(data) => {
                let matches: Vec<Value> = data["matches"].as_array().unwrap_or(&vec![]).iter().map(|m| json!({
                    "segment": m["segment"], "translation": m["translation"],
                    "quality": m["quality"], "source": m["created-by"],
                    "last_updated": m["last-updated-date"]
                })).collect();
                json!({"query": input.text, "langpair": format!("{}|{}", input.source, input.target), "matches": matches.len(), "results": matches}).to_string()
            }
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "List supported languages with codes")]
    async fn list_languages(&self) -> String {
        let langs = vec![
            ("af","Afrikaans"),("am","Amharic"),("ar","Arabic"),("bg","Bulgarian"),("bn","Bengali"),
            ("ca","Catalan"),("cs","Czech"),("da","Danish"),("de","German"),("el","Greek"),
            ("en","English"),("es","Spanish"),("et","Estonian"),("fa","Persian"),("fi","Finnish"),
            ("fr","French"),("ha","Hausa"),("he","Hebrew"),("hi","Hindi"),("hr","Croatian"),
            ("hu","Hungarian"),("id","Indonesian"),("ig","Igbo"),("it","Italian"),("ja","Japanese"),
            ("kn","Kannada"),("ko","Korean"),("lt","Lithuanian"),("lv","Latvian"),("mk","Macedonian"),
            ("ml","Malayalam"),("ms","Malay"),("nl","Dutch"),("no","Norwegian"),("pl","Polish"),
            ("pt","Portuguese"),("ro","Romanian"),("ru","Russian"),("rw","Kinyarwanda"),("sk","Slovak"),
            ("sl","Slovenian"),("sq","Albanian"),("sr","Serbian"),("sv","Swedish"),("sw","Swahili"),
            ("ta","Tamil"),("te","Telugu"),("th","Thai"),("tr","Turkish"),("uk","Ukrainian"),
            ("ur","Urdu"),("vi","Vietnamese"),("yo","Yoruba"),("zh","Chinese"),("zu","Zulu"),
        ];
        let arr: Vec<Value> = langs.iter().map(|(c,n)| json!({"code": c, "name": n})).collect();
        json!({"languages": arr, "total": 200, "note": "Full list at mymemory.translated.net. Use ISO 639-1 codes."}).to_string()
    }
}

adk_mcp_sdk::mcp_2026_server! {
    server: TranslateServer,
    task_tools: ["batch_translate"],
    approval_tools: [],
    cache_ttl_ms: 60_000,
}
