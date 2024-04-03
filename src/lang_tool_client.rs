use languagetool_rust::{CheckRequest, CheckResponse, ServerClient};
use log::{error, info};
use tokio::runtime::Runtime;

#[derive(Debug)]
pub struct LangToolClient {
    pub language: String,
    pub tokio_runtime: Option<Runtime>,
    client: ServerClient,
}

impl LangToolClient {
    pub fn new(lang_tool_url: Option<String>, lang: Option<String>) -> Self {
        let mut language: String = "en-US".to_owned();

        if let Some(lang) = lang {
            language = lang;
        }

        info!("Starting Up Tokio Runtime...");

        let tokio_runtime = Runtime::new();

        let tokio_runtime = match tokio_runtime {
            Ok(tokio_runtime) => Some(tokio_runtime),
            Err(e) => {
                error!("Unable to start up Tokio Runtime {:#?}", e);
                None
            }
        };

        info!("Tokio Runtime has Started");

        return LangToolClient {
            language,
            client: ServerClient::from_env_or_default().with_max_suggestions(20),
            tokio_runtime,
        };
    }

    pub fn get_lang_tool(&self, text: &str) -> Option<CheckResponse> {
        if text.is_empty() {
            return None;
        }

        let mut request = CheckRequest::default().with_text(text.to_owned());
        request = request.with_language(self.language.to_string());

        let tokio_runtime = self
            .tokio_runtime
            .as_ref()
            .expect("This should never panic!");

        // TODO: Should use the check_multiple_and_join on the client!
        let response_task = self.client.check(&request);

        let response = tokio_runtime.block_on(response_task);

        match response {
            Ok(res) => {
                return Some(res);
            }
            Err(e) => {
                error!("Unable to connect to your Language Tool {:#?}", e);
                return None;
            }
        }
    }
}

// TODO: Might need this in the feature. If not remove.
//
// #[derive(Debug)]
// pub struct LangToolClient {
//     pub languagetool_url: String,
//     pub language: String,
//     pub tokio_runtime: Option<Runtime>,
//     client: Client,
// }
//
// impl LangToolClient {
//     pub fn new(lang_tool_url: Option<String>, lang: Option<String>) -> Self {
//         let mut languagetool_url: String = "http://localhost:8081".to_owned();
//         let mut language: String = "en-US".to_owned();
//         let client = Client::new();
//
//         if let Some(url) = lang_tool_url {
//             languagetool_url = url;
//         }
//
//         if let Some(lang) = lang {
//             language = lang;
//         }
//
//         info!("Starting Up Tokio Runtime...");
//
//         let tokio_runtime = Runtime::new();
//
//         let tokio_runtime = match tokio_runtime {
//             Ok(tokio_runtime) => Some(tokio_runtime),
//             Err(e) => {
//                 error!("Unable to start up Tokio Runtime {:#?}", e);
//                 None
//             }
//         };
//
//         info!("Tokio Runtime has Started");
//
//         return LangToolClient {
//             languagetool_url,
//             language,
//             client,
//             tokio_runtime,
//         };
//     }
//
//     pub fn get_lang_tool(&self, text: &str) -> Option<LangTool> {
//         if text.is_empty() {
//             return None;
//         }
//
//         let url = self.languagetool_url.clone() + "/v2/check";
//
//         let tokio_runtime = self
//             .tokio_runtime
//             .as_ref()
//             .expect("This should never panic!");
//
//         let response_task = self
//             .client
//             .post(url)
//             .form(&[("text", text), ("language", self.language.as_str())])
//             .send();
//
//         let response = tokio_runtime.block_on(response_task);
//
//         match response {
//             Ok(res) => {
//                 let status = res.status();
//
//                 //TODO: Need to handler error
//                 let text = tokio_runtime
//                     .block_on(res.text())
//                     .expect("Request error, Unable to get text");
//
//                 if !matches!(status, StatusCode::OK) {
//                     warn!("Something wrong in this text: {}", text);
//                     warn!("Language Tool response: {}", text);
//                     return None;
//                 }
//
//                 // debug!("TEXT: {}", text);
//
//                 //TODO: Need to handler deserializing error
//                 let lang_tool: LangTool = serde_json::from_str(&text).unwrap();
//                 return Some(lang_tool);
//             }
//             Err(e) => {
//                 error!("Unable to connect to your Language Tool {:#?}", e);
//                 return None;
//             }
//         }
//     }
// }
