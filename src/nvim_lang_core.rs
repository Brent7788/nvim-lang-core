use reqwest::Client;

#[derive(Debug)]
pub struct NvimLangCore {
    languagetool_url: String,
    language: String,
    client: Client,
}

impl NvimLangCore {
    fn new(lang_tool_url: Option<String>, lang: Option<String>) -> NvimLangCore {
        let mut languagetool_url: String = "http://localhost:8081".to_owned();
        let mut language: String = "en-US".to_owned();
        let client = Client::new();

        if let Some(url) = lang_tool_url {
            languagetool_url = url;
        }

        if let Some(lang) = lang {
            language = lang;
        }

        return NvimLangCore {
            languagetool_url,
            language,
            client,
        };
    }
}
