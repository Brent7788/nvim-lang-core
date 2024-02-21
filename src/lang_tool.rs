use log::debug;

use crate::{
    lang_tool_client::LangToolClient,
    modules::LangTool,
    programming_lang::{ProgrammingFile, ProgrammingLine},
};

#[derive(Debug)]
pub struct LanguageToolFile<'ltf> {
    pub prog_file: &'ltf ProgrammingFile<'ltf>,
    pub comments: Vec<Comment<'ltf>>,
    pub code: Code<'ltf>,
    pub code_strings: Vec<CodeString<'ltf>>,
}

impl<'ltf> LanguageToolFile<'ltf> {
    pub async fn new(
        prog_file: &'ltf ProgrammingFile<'ltf>,
        client: &LangToolClient,
    ) -> LanguageToolFile<'ltf> {
        return LanguageToolFile {
            prog_file,
            comments: Comment::generate(prog_file, client).await,
            code: Code::generate(prog_file, client).await,
            code_strings: CodeString::generate(prog_file, client).await,
        };
    }
}

#[derive(Debug)]
pub struct Comment<'c> {
    pub prog_lines: Vec<&'c ProgrammingLine>,
    pub line_end_offset: Vec<usize>,
    pub comment: String,
    pub lang_tool: Option<LangTool>,
}

impl<'c> Comment<'c> {
    fn new() -> Comment<'c> {
        return Comment {
            prog_lines: Vec::new(),
            line_end_offset: Vec::new(),
            comment: String::new(),
            lang_tool: None,
        };
    }
    async fn generate<'pl>(
        prog_file: &'pl ProgrammingFile<'pl>,
        client: &LangToolClient,
    ) -> Vec<Comment<'pl>> {
        let mut comments: Vec<Comment> = Vec::new();

        let mut comment: Comment = Comment::new();

        for prog_line in &prog_file.lines {
            if !Comment::is_line_comment(prog_line) && !comment.is_empty() {
                comment.lang_tool = client.get_lang_tool(&comment.comment).await;
                comments.push(comment);
                comment = Comment::new();
                continue;
            }

            if !Comment::is_line_comment(prog_line) && comment.is_empty() {
                continue;
            }

            comment.push_line_end_offset(prog_line);

            comment.comment = format!("{} {}", comment.comment.as_str(), prog_line.get_comment());

            comment.prog_lines.push(prog_line);

            // info!("COMMENT: {:#?}", comment);
        }

        if comment.prog_lines.len() > 0 {
            comment.lang_tool = client.get_lang_tool(&comment.comment).await;
            comments.push(comment);
        }

        // info!("COMMENT: {:#?}", comments);

        return comments;
    }

    fn is_line_comment(prog_line: &ProgrammingLine) -> bool {
        return match prog_line.prog_type {
            crate::programming_lang::ProgrammingLineType::CodeWithComment => true,
            crate::programming_lang::ProgrammingLineType::Comment => true,
            crate::programming_lang::ProgrammingLineType::BlockCommentStart => true,
            crate::programming_lang::ProgrammingLineType::BlockComment => true,
            crate::programming_lang::ProgrammingLineType::BlockCommentEnd => true,
            crate::programming_lang::ProgrammingLineType::BlockCommentStartAndEnd => true,
            _ => false,
        };
    }

    fn push_line_end_offset(&mut self, prog_line: &ProgrammingLine) {
        let last_line_end_offset = match self.line_end_offset.last() {
            Some(ln_end) => ln_end,
            None => &0,
        };

        let offset = prog_line.get_comment().len() + last_line_end_offset;

        self.line_end_offset.push(offset);
    }

    fn is_empty(&self) -> bool {
        if self.prog_lines.is_empty() && self.line_end_offset.is_empty() && self.comment.is_empty()
        {
            return true;
        }

        return false;
    }
}

#[derive(Debug)]
pub struct Code<'c> {
    pub prog_lines: Vec<&'c ProgrammingLine>,
    pub processed_code: String,
    pub lang_tool: Option<LangTool>,
}

impl<'c> Code<'c> {
    // TODO: Need to simplify this method
    async fn generate<'pl>(
        prog_file: &'pl ProgrammingFile<'pl>,
        client: &LangToolClient,
    ) -> Code<'pl> {
        // TODO: Should limit processed char count to 5000, if 5000 create new Code.
        let mut code = Code {
            prog_lines: Vec::with_capacity(prog_file.lines.len()),
            processed_code: String::from("Ignore"),
            lang_tool: None,
        };

        for prog_line in &prog_file.lines {
            if !Code::is_code_line(prog_line) {
                continue;
            }

            let code_line = prog_file
                .lang
                .replase_all_operators_and_syntax_with_whitespace(prog_line.get_code());

            let code_line_split = code_line.split_whitespace();
            let processed_code_len = code.processed_code.len();

            for code_chunk in code_line_split {
                let code_chunk = code_chunk.trim();

                if code_chunk.is_empty() || prog_file.lang.is_reserved_keyword(code_chunk) {
                    continue;
                }

                let code_chunk = prog_file.lang.split_by_naming_conventions(code_chunk);

                code.processed_code.push_str(" ");
                code.processed_code.push_str(code_chunk.trim());
            }

            if processed_code_len < code.processed_code.len() {
                code.prog_lines.push(prog_line);
            }
        }

        if code.processed_code == "Ignore" {
            return code;
        }
        // debug!("CODE: {:#?}", code);

        code.lang_tool = client.get_lang_tool(&code.processed_code).await;

        debug!("CODE: {:#?}", code);

        return code;
    }

    fn is_code_line(prog_line: &ProgrammingLine) -> bool {
        return match prog_line.prog_type {
            crate::programming_lang::ProgrammingLineType::Code => true,
            crate::programming_lang::ProgrammingLineType::CodeWithComment => true,
            _ => false,
        };
    }
}

#[derive(Debug)]
pub struct CodeString<'cs> {
    pub prog_lines: &'cs ProgrammingLine,
    pub lang_tool: Option<LangTool>,
}

impl<'cs> CodeString<'cs> {
    async fn generate(
        prog_file: &'cs ProgrammingFile<'cs>,
        client: &LangToolClient,
    ) -> Vec<CodeString<'cs>> {
        unimplemented!();
    }

    fn is_code_string_line(prog_line: &ProgrammingLine) -> bool {
        return match prog_line.prog_type {
            crate::programming_lang::ProgrammingLineType::CodeWithString => true,
            crate::programming_lang::ProgrammingLineType::CodeWithStringWithComment => true,
            _ => false,
        };
    }
}
