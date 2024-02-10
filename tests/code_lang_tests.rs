#[cfg(test)]
pub mod code_lang_tests {
    use std::env;

    use log::info;
    use nvim_lang_core::{
        common::{
            logger::Logger,
            test::{get_test_code_path, Expected},
        },
        nvim_lang_core::NvimLangCore,
    };

    #[tokio::test]
    async fn simple_code_should_be() {
        env::set_var("RUST_BACKTRACE", "1");

        let file_path = get_test_code_path("/simple_code.rs");

        let core = NvimLangCore::new(None, None);

        let result = core.process_file(file_path).await;

        // info!("{:#?}", result);

        log::logger().flush();

        Expected::data_len_to_be(2, &result);
        Expected::new(1, 7, 15, 1, "upercase", vec!["uppercase"]).assert(0, &result);
        Expected::new(1, 16, 22, 2, "prduct", vec!["product", "pr duct"]).assert(1, &result);
    }

    #[tokio::test]
    async fn multiple_code_should_be() {
        Logger::console_init();
        env::set_var("RUST_BACKTRACE", "1");

        let file_path = get_test_code_path("/multiple_code.rs");

        let core = NvimLangCore::new(None, None);

        let result = core.process_file(file_path).await;

        info!("{:#?}", result);

        log::logger().flush();

        Expected::data_len_to_be(12, &result);
        // Expected::new(1, 7, 15, 1, "upercase", vec!["uppercase"]).assert(0, &result);
        // Expected::new(1, 16, 22, 2, "prduct", vec!["product", "pr duct"]).assert(1, &result);
    }
}
