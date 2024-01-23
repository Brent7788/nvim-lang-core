#[derive(Debug)]
pub enum ProgrammingLanguageType {
    Lua,
    Rust,
}

#[derive(Debug)]
pub struct ProgrammingLanguage {
    extension: String,
    comment_delimiter: String,
    lang_type: ProgrammingLanguageType,
}

impl ProgrammingLanguage {
    pub fn init() -> Vec<ProgrammingLanguage> {
        let mut programming_languages: Vec<ProgrammingLanguage> = Vec::with_capacity(2);

        programming_languages.push(ProgrammingLanguage {
            extension: ".lua".to_owned(),
            comment_delimiter: "--".to_owned(),
            lang_type: ProgrammingLanguageType::Lua,
        });

        return programming_languages;
    }
}

#[derive(Debug)]
pub struct ProgrammingFile {}
