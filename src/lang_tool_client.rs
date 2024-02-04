use log::{debug, error, info, warn};
use reqwest::{Client, StatusCode};

use crate::modules::LangTool;

#[derive(Debug)]
pub struct LangToolClient {
    pub languagetool_url: String,
    pub language: String,
    pub client: Client,
}

impl LangToolClient {
    pub fn new(lang_tool_url: Option<String>, lang: Option<String>) -> Self {
        let mut languagetool_url: String = "http://localhost:8081".to_owned();
        let mut language: String = "en-US".to_owned();
        let client = Client::new();

        if let Some(url) = lang_tool_url {
            languagetool_url = url;
        }

        if let Some(lang) = lang {
            language = lang;
        }

        return LangToolClient {
            languagetool_url,
            language,
            client,
        };
    }

    pub async fn get_lang_tool(&self, text: &str) -> Option<LangTool> {
        if text.is_empty() {
            return None;
        }

        let url = self.languagetool_url.clone() + "/v2/check";
        let res = self
            .client
            .post(url)
            .form(&[("text", text), ("language", self.language.as_str())])
            .send()
            .await;

        match res {
            Ok(res) => {
                let status = res.status();

                //TODO: Need to handler error
                let text = res.text().await.expect("Request error, Unable to get text");

                if !matches!(status, StatusCode::OK) {
                    warn!("Something wrong in this text: {}", text);
                    warn!("Language Tool response: {}", text);
                    return None;
                }

                // debug!("TEXT: {}", text);

                //TODO: Need to handler deserializing error
                let lang_tool: LangTool = serde_json::from_str(&text).unwrap();
                return Some(lang_tool);
            }
            Err(e) => {
                error!("Unable to connect to your Language Tool {:#?}", e);
                return None;
            }
        }
    }
}
