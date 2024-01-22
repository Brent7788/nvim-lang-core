use serde::Deserialize;

// TODO: Remember to remove all of the '#[allow(dead_code)]'

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct LangTool {
    matches: Vec<Matche>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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
#[allow(dead_code)]
pub struct Replacement {
    value: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Rule {
    id: String,
}
