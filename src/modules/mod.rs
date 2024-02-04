use serde::Deserialize;

// TODO: Remember to remove all of the '#[allow(dead_code)]'

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct LangTool {
    pub matches: Vec<Matche>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Matche {
    pub message: String,
    #[serde(rename = "shortMessage")]
    pub short_message: String,
    pub replacements: Vec<Replacement>,
    pub offset: usize,
    pub length: usize,
    pub context: Context,
    pub sentence: String,
    pub rule: Rule,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Replacement {
    pub value: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Context {
    pub text: String,
    pub offset: usize,
    pub length: usize,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Rule {
    id: String,
    pub category: Category,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Category {
    pub id: String,
}
