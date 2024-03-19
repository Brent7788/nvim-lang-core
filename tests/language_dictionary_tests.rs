#[cfg(test)]
pub mod language_dictionary_tests {
    use std::env;

    use log::info;
    use nvim_lang_core::{common::logger::Logger, nvim_lang_dictionary::NvimLanguageDictionary};

    // WARN: cargo watch loops forever, when running this test in watch mode!
    #[test]
    fn add_remove_should_be() {
        env::set_var("RUST_BACKTRACE", "1");
        Logger::console_init();

        let mut nvim_language_dictionary = NvimLanguageDictionary::new();

        info!("{:#?}", nvim_language_dictionary);

        assert_eq!(2, nvim_language_dictionary.get_words().len());

        nvim_language_dictionary.append_word("nvim".to_owned());

        let words = nvim_language_dictionary.get_words();

        assert_eq!(3, words.len());
        assert_eq!("nvim".to_owned(), words[2]);

        info!("{:#?}", nvim_language_dictionary);
        nvim_language_dictionary.remove_word("nvim".to_owned());

        let words = nvim_language_dictionary.get_words();

        assert_eq!(2, words.len());

        log::logger().flush();
    }
}
