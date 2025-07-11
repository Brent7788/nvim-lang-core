#[cfg(test)]
pub mod language_dictionary_tests {
    use log::debug;
    use nvim_lang_core::{
        code::code_file::CodeFile,
        common::{logger::Logger, test::get_bench_path},
        nvim_lang_dictionary::NvimLanguageDictionary,
        nvim_language::core::NvimLanguageCore,
    };
    use std::{env, sync::Arc};
    use tokio::runtime::Runtime;

    #[test]
    fn add_remove_should_be() {
        env::set_var("RUST_BACKTRACE", "1");

        let mut nvim_language_dictionary = NvimLanguageDictionary::new(true);

        nvim_language_dictionary.append_word("lang".to_owned());
        nvim_language_dictionary.append_word("tokio".to_owned());
        nvim_language_dictionary.remove_word("tokio".to_owned());
        nvim_language_dictionary.append_word("tokio".to_owned());

        assert_eq!(2, nvim_language_dictionary.get_words().len());

        nvim_language_dictionary.append_word("nvim".to_owned());

        let words = nvim_language_dictionary.get_words();

        assert_eq!(3, words.len());
        assert_eq!("nvim".to_owned(), words[2]);

        nvim_language_dictionary.remove_word("tokio".to_owned());

        let words = nvim_language_dictionary.get_words();

        assert_eq!(2, words.len());
        assert_eq!("lang".to_owned(), words[0]);
        assert_eq!("nvim".to_owned(), words[1]);

        nvim_language_dictionary.remove_word("nvim".to_owned());
    }
}
