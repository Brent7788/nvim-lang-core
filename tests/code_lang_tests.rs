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

        // info!("{:#?}", result);

        log::logger().flush();

        Expected::data_len_to_be(12, &result);
        Expected::new(2, 15, 20, 3, "Foldr", vec!["Fold", "Folder", "Folds"]).assert(0, &result);
        Expected::new(6, 9, 14, 3, "Foldr", vec!["Fold", "Folder", "Folds"]).assert(1, &result);
        Expected::new(7, 62, 67, 3, "Foldr", vec!["Fold", "Folder", "Folds"]).assert(2, &result);
        Expected::new(12, 19, 24, 3, "Foldr", vec!["Fold", "Folder", "Folds"]).assert(3, &result);
        Expected::new(7, 11, 18, 2, "generte", vec!["generate", "gene rte"]).assert(4, &result);
        Expected::new(3, 4, 9, 3, "foldr", vec!["fold", "folder", "folds"]).assert(5, &result);
        Expected::new(7, 19, 24, 3, "foldr", vec!["fold", "folder", "folds"]).assert(6, &result);
        Expected::new(12, 27, 32, 3, "foldr", vec!["fold", "folder", "folds"]).assert(7, &result);
        Expected::new(7, 25, 31, 1, "systim", vec!["system"]).assert(8, &result);
        Expected::new(12, 39, 45, 1, "systim", vec!["system"]).assert(9, &result);
        Expected::new(
            7,
            41,
            48,
            18,
            "procces",
            vec!["process", "produces", "prices"],
        )
        .assert(10, &result);
        Expected::new(
            8,
            11,
            18,
            18,
            "procces",
            vec!["process", "produces", "prices"],
        )
        .assert(11, &result);
    }
}
