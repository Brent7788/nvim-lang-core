#[cfg(test)]
pub mod first_test {
    use std::env;

    use log::{error, info};
    use nvim_lang_core::{common::logger::Logger, modules::LangTool, nvim_lang_core::NvimLangCore};
    use reqwest::Client;

    //curl -X POST --header 'Content-Type: application/x-www-form-urlencoded'
    //--header 'Accept: application/json' -d 'language=en-US&enabledOnly=false' 'http://localhost:8081/v2/check?text=get%20this%20tets'

    #[tokio::test]
    async fn first() {
        Logger::console_init();
        env::set_var("RUST_BACKTRACE", "1");

        info!("First test running...");

        let client = Client::new();

        let res = client
            .post("http://localhost:8081/v2/check?language=en-US")
            .form(&[("text", "get this thig")])
            .send()
            .await;

        match res {
            Ok(r) => {
                info!("Request was send");
                let status = r.status();
                info!("Request status {:?}", status);
                let text = r.text().await.expect("Unable to get text");
                let des: LangTool = serde_json::from_str(&text).unwrap();
                info!("Request object {:#?}", &des);
            }
            Err(e) => {
                error!("Request error {:?}", e);
            }
        }

        log::logger().flush();
    }

    #[tokio::test]
    async fn sec() {
        let core = NvimLangCore::new(None, None);

        core.process_file(
            "/home/brent/Documents/projects/nvim-lang-core/tests/file_test_cases/person.rs"
                .to_owned(),
        );
    }
}
