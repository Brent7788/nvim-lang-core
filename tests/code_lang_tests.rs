#[cfg(test)]
pub mod code_lang_tests {
    use std::env;

    use log::info;
    use nvim_lang_core::{
        common::{
            logger::Logger,
            test::{get_test_code_path, get_test_comment_path, Expected},
        },
        nvim_lang_core::NvimLangCore,
    };

    #[tokio::test]
    async fn simple_code_should_be() {
        Logger::console_init();
        env::set_var("RUST_BACKTRACE", "1");

        let file_path = get_test_code_path("/simple_code.rs");

        let core = NvimLangCore::new(None, None);

        let result = core.process_file(file_path).await;

        info!("{:#?}", result);

        log::logger().flush();

        /*         Expected::data_len_to_be(15, &result);
        Expected::new(3, 34, 42, 1, "too have", vec!["to have"]).assert(0, &result);
        Expected::new(4, 43, 50, 1, "colours", vec!["colors"]).assert(1, &result);
        Expected::new(4, 65, 73, 2, "seplling", vec!["selling", "spelling"]).assert(2, &result);
        Expected::new(4, 90, 100, 1, "underilnes", vec!["underlines"]).assert(3, &result);
        Expected::new(5, 4, 15, 1, "Furthermore", vec!["Furthermore,"]).assert(4, &result);
        Expected::new(5, 24, 31, 1, "error's", vec!["errors"]).assert(5, &result);
        Expected::new(6, 41, 61, 1, "in a reliable manner", vec!["reliably"]).assert(6, &result);
        Expected::new(7, 4, 7, 1, "did", vec!["Did"]).assert(7, &result);
        Expected::new(7, 46, 61, 1, "double clicking", vec!["double-clicking"]).assert(8, &result);
        Expected::new(7, 70, 73, 1, "Its", vec!["It's"]).assert(9, &result);
        Expected::new(7, 74, 75, 1, "a", vec!["an"]).assert(10, &result);
        Expected::new(8, 33, 37, 1, "youd", vec!["you'd"]).assert(11, &result);
        Expected::new(8, 68, 78, 1, "over sea's", vec!["overseas"]).assert(12, &result);
        Expected::new(9, 18, 37, 1, "PM in the afternoon", vec!["PM"]).assert(13, &result);
        Expected::new(9, 41, 60, 0, "Monday, 27 May 2007", vec![]).assert(14, &result); */
    }
}
