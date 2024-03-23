#[cfg(test)]
pub mod language_dictionary_tests {
    use log::info;
    use nvim_lang_core::{common::logger::Logger, nvim_lang_dictionary::NvimLanguageDictionary};
    use std::env;

    #[test]
    fn add_remove_should_be() {
        env::set_var("RUST_BACKTRACE", "1");
        Logger::console_init();

        let mut nvim_language_dictionary = NvimLanguageDictionary::new();

        nvim_language_dictionary.append_word("lang".to_owned());
        nvim_language_dictionary.append_word("tokio".to_owned());
        nvim_language_dictionary.remove_word("tokio".to_owned());
        nvim_language_dictionary.append_word("tokio".to_owned());

        info!("{:#?}", nvim_language_dictionary);
        log::logger().flush();
        assert_eq!(2, nvim_language_dictionary.get_words().len());

        nvim_language_dictionary.append_word("nvim".to_owned());

        let words = nvim_language_dictionary.get_words();

        info!("{:#?}", nvim_language_dictionary);
        info!("{:#?}", words);
        log::logger().flush();
        assert_eq!(3, words.len());
        assert_eq!("nvim".to_owned(), words[2]);

        nvim_language_dictionary.remove_word("tokio".to_owned());

        let words = nvim_language_dictionary.get_words();

        assert_eq!(2, words.len());
        assert_eq!("lang".to_owned(), words[0]);
        assert_eq!("nvim".to_owned(), words[1]);

        nvim_language_dictionary.remove_word("nvim".to_owned());

        log::logger().flush();
    }
}
