#[cfg(test)]
pub mod code_string_tests {
    use std::env;

    use log::debug;
    use nvim_lang_core::{
        common::{
            logger::Logger,
            test::{get_test_code_string_path, Expected},
        },
        nvim_lang_core::NvimLangCore,
    };

    #[test]
    fn simple_string_should_be() {
        env::set_var("RUST_BACKTRACE", "1");

        let file_path = get_test_code_string_path("/simple_string.rs");

        let core = NvimLangCore::new(None, None);

        let result = core.process_file(file_path);

        Expected::data_len_to_be(4, &result);
        Expected::new(2, 5, 6, 1, "a", vec!["A"]).assert(0, &result);
        Expected::new(2, 17, 24, 5, "brances", vec!["branches"]).assert(1, &result);
        Expected::new(2, 41, 50, 2, "especialy", vec!["especially"]).assert(2, &result);
        Expected::new(2, 82, 88, 2, "prduct", vec!["product"]).assert(3, &result);
    }

    #[test]
    fn multiple_strings_should_be() {
        Logger::console_init();
        env::set_var("RUST_BACKTRACE", "1");

        let file_path = get_test_code_string_path("/multiple_strings.rs");

        let core = NvimLangCore::new(None, None);

        let result = core.process_file(file_path);

        debug!("{:#?}", result);
        log::logger().flush();

        Expected::data_len_to_be(5, &result);
        Expected::new(2, 8, 14, 2, "prduct", vec!["product", "pr duct"]).assert(0, &result);
        Expected::new(2, 20, 29, 1, "oparation", vec!["operation"]).assert(1, &result);
        Expected::new(2, 35, 41, 2, "purson", vec!["person", "parson"]).assert(2, &result);
        Expected::new(3, 21, 30, 1, "OPARATION", vec!["OPERATION"]).assert(3, &result);
        Expected::new(3, 31, 37, 1, "PRDUCT", vec!["PRODUCT"]).assert(4, &result);
    }
}
