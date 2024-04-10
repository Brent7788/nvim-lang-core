use std::{
    process::Command,
    str::from_utf8,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use languagetool_rust::{error::Result, CheckRequest, CheckResponse, ServerClient};
use log::{error, info, warn};
use tokio::runtime::Runtime;

#[derive(Debug)]
pub struct LangToolClient {
    pub language: String,
    pub tokio_runtime: Option<Runtime>,
    client: Arc<Mutex<ServerClient>>,
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
            client: Arc::new(Mutex::new(get_language_tool_client(&tokio_runtime))),
            tokio_runtime,
        };
    }

    pub fn get_internal_client_clone(&self) -> Arc<Mutex<ServerClient>> {
        return self.client.clone();
    }

    pub fn docker_setup(&self) {
        let tokio_runtime = match &self.tokio_runtime {
            Some(tokio_runtime) => tokio_runtime,
            None => return,
        };

        if !languagetool_docker_image_exit() {
            self.language_tool_docker_pull();
        }

        docker_language_tool_start();

        let client = ServerClient::new("http://localhost", "8010");

        if let Result::Ok(_) = tokio_runtime.block_on(client.ping()) {
            info!("Custom Language Tool server already setup.");
            return;
        }

        multiple_ping(tokio_runtime, &client);

        match tokio_runtime.block_on(client.ping()) {
            Ok(_) => {
                info!("Set local language tool client");
                let mut current_client = self.client.lock().unwrap();

                *current_client = client
            }
            Err(e) => warn!(
                "Was unable to set local language tool client. Error: {:#?}",
                e
            ),
        };
    }

    fn language_tool_docker_pull(&self) {
        let cargo_languagetool_cli = Command::new("cargo")
            .args(["install", "languagetool-rust", "--features", "full"])
            .output();

        match cargo_languagetool_cli {
            Ok(output) => info!("cargo install languagetoo-rust output: {:#?}", output),
            Err(e) => {
                error!(
                    "Unable to install languagetool-rust CLI using cargo. Error: {:#?}",
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
                error!("Unable to docker pull LanguageTool server. Error: {:#?}", e);
                return;
            }
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

        // WARN: This thread lock will significant to reduce the method speed.
        // TODO: Should use the check_multiple_and_join on the client!
        let response = match self.client.lock() {
            Ok(client) => tokio_runtime.block_on(client.check(&request)),
            Err(e) => {
                error!(
                    "LanguageTool client was poisoned, reverting to default client. Error: {:#?}",
                    e
                );

                // TODO: Should try to recover out of poison error
                tokio_runtime.block_on(ServerClient::default().check(&request))
            }
        };

        match response {
            Ok(res) => {
                return Some(res);
            }
            Err(e) => {
                error!("Unable to connect to your LanguageTool {:#?}", e);
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
        info!("Using default LanguageTool client");
        return ServerClient::default();
    }

    return client;
}

// TODO: Naming
fn multiple_ping(tokio_runtime: &Runtime, client: &ServerClient) {
    for _ in 1..10 {
        info!("Pinning http://localhost:8010 ...");
        if let Result::Ok(_) = tokio_runtime.block_on(client.ping()) {
            info!("End of pinning http://localhost:8010");
            break;
        }
        thread::sleep(Duration::from_millis(300));
    }
}

fn docker_language_tool_start() {
    let docker_start_output = Command::new("ltrs").args(["docker", "start"]).output();

    match docker_start_output {
        Ok(output) => {
            if output.stdout.len() > 0 {
                info!(
                    "Successfully started docker LanguageTool server. Output: {:#?}",
                    output.stdout
                );
                return;
            }

            warn!("Unknown ltrs docker start. Output: {:#?}", output);
        }
        Err(e) => {
            warn!(
                "Unable to start LanguageTool server using docker. Error: {:#?}",
                e
            );
            return;
        }
    };
}

fn languagetool_docker_image_exit() -> bool {
    let docker_images_output = Command::new("docker").arg("images").output();

    match docker_images_output {
        Ok(output) => {
            if output.stdout.len() == 0 {
                return false;
            }

            let stdout = &output.stdout[0..output.stdout.len()];
            let stdout = match from_utf8(stdout) {
                Ok(stdout) => stdout,
                Err(e) => {
                    error!(
                        "Utf8 conversion error after running `docker images` command. Error: {:#?}",
                        e
                    );
                    return false;
                }
            };

            if !stdout.contains("erikvl87/languagetool") {
                warn!("Docker image erikvl87/languagetool has not been installed yet!");
                return false;
            }

            info!(
                "Docker image erikvl87/languagetool has been installed. Output{:#?}",
                output
            );
            return true;
        }
        Err(e) => {
            warn!("Unable to run docker images. Error: {:#?}", e);
            return false;
        }
    };
}
