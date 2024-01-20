use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LangTool {
    matches: Vec<Matche>,
}

#[derive(Deserialize, Debug)]
pub struct Matche {
    message: String,
    #[serde(rename = "shortMessage")]
    short_message: String,
    replacements: Vec<Replacement>,
    offset: u16,
    length: u16,
    sentence: String,
    rule: Rule,
}

#[derive(Deserialize, Debug)]
pub struct Replacement {
    value: String,
}

#[derive(Deserialize, Debug)]
pub struct Rule {
    id: String,
}
