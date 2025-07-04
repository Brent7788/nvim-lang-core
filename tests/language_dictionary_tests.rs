#[cfg(test)]
pub mod language_dictionary_tests {
    use nvim_lang_core::{
        code::{
            code_file::CodeFile,
            programming::{ProgrammingLanguage, RUST},
        },
        common::logger::Logger,
        nvim_lang_dictionary::NvimLanguageDictionary,
        programming_lang::ProgrammingFile,
    };
    use std::{env, time::Duration};

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
