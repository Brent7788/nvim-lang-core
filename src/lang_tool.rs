use crate::{modules::LangTool, programming_lang::ProgrammingLine};

//TODO: Find better name
#[derive(Debug)]
pub struct LangToolCore {}

#[derive(Debug)]
struct Comment {
    prog_lines: Vec<ProgrammingLine>,
    lang_tool: LangTool,
}
