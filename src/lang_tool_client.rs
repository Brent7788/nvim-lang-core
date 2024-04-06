use std::{
    process::Command,
    str::{from_utf8, from_utf8_unchecked},
};

use languagetool_rust::{error::Result, CheckRequest, CheckResponse, ServerClient};
use log::{error, info, warn};
use tokio::runtime::Runtime;

#[derive(Debug)]
pub struct LangToolClient {
    pub language: String,
    pub tokio_runtime: Option<Runtime>,
    client: ServerClient,
}

impl LangToolClient {
    pub fn new(_lang_tool_url: Option<String>, lang: Option<String>) -> Self {
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
            client: get_language_tool_client(&tokio_runtime),
            tokio_runtime,
        };
    }

    pub fn docker_setup(&self) {
        let tokio_runtime = match &self.tokio_runtime {
            Some(tokio_runtime) => tokio_runtime,
            None => return,
        };

        let client = ServerClient::new("http://localhost", "8010");

        if let Result::Ok(_) = tokio_runtime.block_on(client.ping()) {
            return;
        }

        let cargo_languagetool_cli = Command::new("cargo")
            .args(["install", "languagetool-rust", "--features", "full"])
            .output();

        match cargo_languagetool_cli {
            Ok(output) => info!("cargo install languagetoo-rust output: {:#?}", output),
            Err(e) => {
                error!(
                    "Unable to install languagetool-rust cli using cargo. Error: {:#?}",
                    e
                );
                return;
            }
        };

        let docker_pull_output = Command::new("ltrs").args(["docker", "pull"]).output();

        match docker_pull_output {
            Ok(output) => {
                info!("ltrs docker pull. Output: {:#?}", output);
            }
            Err(e) => {
                error!(
                    "Unable to docker pull language tool server. Error: {:#?}",
                    e
                );
                return;
            }
        };

        docker_language_tool_start();

        // TODO: Set new Servire Client
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

fn get_language_tool_client(tokio_runtime: &Option<Runtime>) -> ServerClient {
    let tokio_runtime = match tokio_runtime {
        Some(tokio_runtime) => tokio_runtime,
        None => return ServerClient::default(),
    };

    let client = ServerClient::new("http://localhost", "8010");

    if let Result::Err(_) = tokio_runtime.block_on(client.ping()) {
        docker_language_tool_start();
    }

    if let Result::Err(e) = tokio_runtime.block_on(client.ping()) {
        warn!("Unable to start Language Tool Client. Pinning Language Tool Server fialed with this error: {:#?}", e);
        return ServerClient::default();
    }

    return client;
}

fn docker_language_tool_start() {
    let docker_images_output = Command::new("docker").arg("images").output();

    match docker_images_output {
        Ok(output) => {
            if output.stdout.len() == 0 {
                return;
            }

            let stdout = &output.stdout[0..output.stdout.len()];
            let stdout = match from_utf8(stdout) {
                Ok(stdout) => stdout,
                Err(e) => {
                    error!("Utf8 converstion error after running `docker images` command. Error: {:#?}", e);
                    return;
                }
            };

            if !stdout.contains("erikvl87/languagetool") {
                warn!("Docker image erikvl87/languagetool has not been installed yet!");
                return;
            }

            info!("{:#?}", output);
        }
        Err(e) => {
            warn!("Unable to run docker images. Error: {:#?}", e);
            return;
        }
    };

    let docker_start_output = Command::new("ltrs").args(["docker", "start"]).output();

    match docker_start_output {
        Ok(output) => {
            if output.stdout.len() > 0 {
                info!(
                    "Succefulty started language tool server. Output: {:#?}",
                    output.stdout
                );
                return;
            }

            warn!("Unknown ltrs docker start. Output: {:#?}", output);
        }
        Err(e) => {
            warn!(
                "Unable to start language tool server using docker. Error: {:#?}",
                e
            );
            return;
        }
    };
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
