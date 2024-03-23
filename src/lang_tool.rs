use std::sync::MutexGuard;

use log::debug;

use crate::{
    lang_tool_client::LangToolClient,
    modules::{Context, LangTool, Matche},
    nvim_lang_dictionary::NvimLanguageDictionary,
    programming_lang::{ProgrammingFile, ProgrammingLine},
};

pub trait LangToolTrait {
    fn get_matches(&self) -> Option<&Vec<Matche>>;
}

impl LangToolTrait for Option<LangTool> {
    fn get_matches(&self) -> Option<&Vec<Matche>> {
        return match self {
            Some(ref lang_tool) => {
                if lang_tool.matches.is_empty() {
                    return None;
                }

                return Some(&lang_tool.matches);
            }
            None => None,
        };
    }
}

pub trait LangTooContextTrait {
    fn get_incorrect_chunk(&self) -> &str;
}

impl LangTooContextTrait for Context {
    fn get_incorrect_chunk(&self) -> &str {
        let offset = self.offset;
        let length = self.offset + self.length;
        return &self.text[offset..length];
    }
}

#[derive(Debug)]
pub struct LanguageToolFile<'ltf> {
    pub prog_file: &'ltf ProgrammingFile<'ltf>,
    pub lines: Vec<LanguageToolLines<'ltf>>,
}

impl<'ltf> LanguageToolFile<'ltf> {
    pub fn new(
        prog_file: &'ltf ProgrammingFile<'ltf>,
        language_dictionary: &Option<MutexGuard<NvimLanguageDictionary>>,
        client: &LangToolClient,
    ) -> LanguageToolFile<'ltf> {
        return LanguageToolFile {
            prog_file,
            lines: LanguageToolLines::generate(prog_file, language_dictionary, client),
        };
    }
}

#[derive(Debug)]
pub enum LanguageToolLinesType {
    Comment,
    Code,
    String,
    Undefined,
}

#[derive(Debug)]
pub struct LanguageToolLines<'ltl> {
    // TODO: Need to find a way to use Vec::with_capacity.
    //       Maybe on the ProgrammingFile predetermine/count comment, code and string line
    pub prog_lines: Vec<&'ltl ProgrammingLine>,
    pub line_end_offset: Vec<usize>,
    pub lang_tool: Option<LangTool>,
    pub tp: LanguageToolLinesType,
}

impl<'ltl> LanguageToolLines<'ltl> {
    fn generate(
        prog_file: &'ltl ProgrammingFile<'ltl>,
        language_dictionary: &Option<MutexGuard<NvimLanguageDictionary>>,
        client: &LangToolClient,
    ) -> Vec<LanguageToolLines<'ltl>> {
        const CODE_COUNT: u64 = 1;
        let lang_tool_lines_count =
            (CODE_COUNT + prog_file.commet_count + prog_file.string_count) as usize;

        let mut lang_tool_lines: Vec<LanguageToolLines> = Vec::with_capacity(lang_tool_lines_count);

        lang_tool_lines.push_if_comments(prog_file, client);
        lang_tool_lines.push_if_code(prog_file, language_dictionary, client);
        lang_tool_lines.push_if_strings(prog_file, client);

        return lang_tool_lines;
    }

    fn push_line_end_offset(&mut self, prog_raw_line_length: usize) {
        let last_line_end_offset = match self.line_end_offset.last() {
            Some(ln_end) => ln_end,
            None => &0,
        };

        let offset = prog_raw_line_length + last_line_end_offset;

        self.line_end_offset.push(offset);
    }

    fn is_comment_empty(&self, full_comment: &str) -> bool {
        if self.prog_lines.is_empty() && self.line_end_offset.is_empty() && full_comment.is_empty()
        {
            return true;
        }

        return false;
    }
}

trait LanguageToolLinesVecTrait<'ltl> {
    fn push_if_comments(&mut self, prog_file: &'ltl ProgrammingFile<'ltl>, client: &LangToolClient);
    fn push_if_code(
        &mut self,
        prog_file: &'ltl ProgrammingFile<'ltl>,
        language_dictionary: &Option<MutexGuard<NvimLanguageDictionary>>,
        client: &LangToolClient,
    );
    fn push_if_strings(&mut self, prog_file: &'ltl ProgrammingFile<'ltl>, client: &LangToolClient);
}

impl<'ltl> LanguageToolLinesVecTrait<'ltl> for Vec<LanguageToolLines<'ltl>> {
    fn push_if_comments(
        &mut self,
        prog_file: &'ltl ProgrammingFile<'ltl>,
        client: &LangToolClient,
    ) {
        // TODO: Need to find a way to use Vec::with_capacity.
        //       Maybe on the ProgrammingFile predetermine/count comment, code and string line
        let mut comment: LanguageToolLines = LanguageToolLines {
            prog_lines: Vec::new(),
            line_end_offset: Vec::new(),
            lang_tool: None,
            tp: LanguageToolLinesType::Undefined,
        };
        let mut full_comment = String::new();

        for prog_line in &prog_file.lines {
            if !prog_line.is_line_comment() && !comment.is_comment_empty(&full_comment) {
                comment.lang_tool = client.get_lang_tool(&full_comment);
                comment.tp = LanguageToolLinesType::Comment;
                self.push(comment);

                full_comment = String::new();
                comment = LanguageToolLines {
                    prog_lines: Vec::new(),
                    line_end_offset: Vec::new(),
                    lang_tool: None,
                    tp: LanguageToolLinesType::Undefined,
                };
                continue;
            }

            if !prog_line.is_line_comment() && comment.is_comment_empty(&full_comment) {
                continue;
            }

            let prog_line_comment = prog_line.get_comment();
            comment.push_line_end_offset(prog_line_comment.len());

            full_comment = format!("{} {}", full_comment.as_str(), prog_line_comment);

            comment.prog_lines.push(prog_line);

            // info!("COMMENT: {:#?}", comment);
        }

        if comment.prog_lines.len() > 0 {
            comment.tp = LanguageToolLinesType::Comment;
            comment.lang_tool = client.get_lang_tool(&full_comment);
            self.push(comment);
        }
    }

    // TODO: This method does a lot of string(that is on the heap) transformation.
    //       Find away to do this on the stack.
    fn push_if_code(
        &mut self,
        prog_file: &'ltl ProgrammingFile<'ltl>,
        language_dictionary: &Option<MutexGuard<NvimLanguageDictionary>>,
        client: &LangToolClient,
    ) {
        // TODO: Should limit processed char count to 5000, if 5000 create new Code.
        // TODO: Need to find a way to use Vec::with_capacity.
        //       Maybe on the Programming File predetermine/count comment, code and string line
        let mut code: LanguageToolLines = LanguageToolLines {
            prog_lines: Vec::with_capacity(prog_file.lines.len()),
            line_end_offset: Vec::with_capacity(0),
            lang_tool: None,
            tp: LanguageToolLinesType::Undefined,
        };

        let mut processed_code = String::from("Ignore");

        for prog_line in &prog_file.lines {
            if !prog_line.is_code_line() {
                continue;
            }

            let code_line = prog_file
                .lang
                .replase_all_operators_and_syntax_with_whitespace(prog_line.get_code());

            let code_line_split = code_line.split_whitespace();
            let processed_code_len = processed_code.len();

            for code_chunk in code_line_split {
                let code_chunk = code_chunk.trim();

                if code_chunk.is_empty()
                    || code_chunk.len() == 1
                    || prog_file.lang.is_reserved_keyword(code_chunk)
                {
                    continue;
                }

                let code_chunk = prog_file.lang.split_by_naming_conventions(code_chunk);

                processed_code.push_str(" ");
                processed_code.push_str(code_chunk.trim());
            }

            if processed_code_len < processed_code.len() {
                code.prog_lines.push(prog_line);
            }
        }

        if processed_code == "Ignore" {
            return;
        }

        if let Some(language_dictionary) = language_dictionary {
            processed_code = language_dictionary.replase_with_dictionary_values(processed_code)
        }

        // debug!("CODE: {:#?}", processed_code);

        code.lang_tool = client.get_lang_tool(&processed_code);

        // debug!("CODE: {:#?}", code);

        code.tp = LanguageToolLinesType::Code;
        self.push(code);
    }

    fn push_if_strings(&mut self, prog_file: &'ltl ProgrammingFile<'ltl>, client: &LangToolClient) {
        for line in &prog_file.lines {
            if !line.is_code_string_line() {
                continue;
            }

            for str_line_opt in &line.string_line {
                let str_line = match str_line_opt {
                    Some(str_line) => str_line,
                    None => break,
                };

                if str_line.is_empty() {
                    continue;
                }

                self.push(LanguageToolLines {
                    prog_lines: vec![line],
                    line_end_offset: Vec::with_capacity(0),
                    lang_tool: client.get_lang_tool(str_line),
                    tp: LanguageToolLinesType::String,
                });
            }
        }
    }
}
