use serde::Deserialize;

// TODO: Remember to remove all of the '#[allow(dead_code)]'
// All of the u16 might need to be bigger.

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
    context: Context,
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
pub struct Context {
    text: String,
    offset: u16,
    length: u16,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Rule {
    id: String,
}
