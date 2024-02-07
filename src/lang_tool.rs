use crate::{
    lang_tool_client::LangToolClient,
    modules::LangTool,
    programming_lang::{ProgrammingFile, ProgrammingLine},
};

#[derive(Debug)]
pub struct LanguageToolFile<'ltf> {
    pub prog_file: &'ltf ProgrammingFile<'ltf>,
    pub comments: Vec<Comment<'ltf>>,
}

impl<'ltf> LanguageToolFile<'ltf> {
    pub async fn new(
        prog_file: &'ltf ProgrammingFile<'ltf>,
        client: &LangToolClient,
    ) -> LanguageToolFile<'ltf> {
        return LanguageToolFile {
            prog_file,
            comments: Comment::generate(prog_file, client).await,
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
            } else if !Comment::is_line_comment(prog_line) && comment.is_empty() {
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
    pub prog_line: &'c ProgrammingLine,
    pub processed_code_line: String,
    pub lang_tool: Option<LangTool>,
}

impl<'c> Code<'c> {
    async fn generate<'pl>(
        prog_file: &'pl ProgrammingFile<'pl>,
        client: &LangToolClient,
    ) -> Vec<Code<'pl>> {
        let mut code_lines: Vec<Code> = Vec::new();

        for prog_line in &prog_file.lines {
            if !Code::is_code_line(prog_line) {
                continue;
            }

            let mut code = Code {
                prog_line,
                processed_code_line: prog_file
                    .lang
                    .replase_all_sytex_with_empty_space(prog_line.get_code()),
                lang_tool: None,
            };

            let code_line_split = code.processed_code_line.split_whitespace();
            let mut n = String::new();
            for code_chunk in code_line_split {
                if code_chunk.is_empty() {
                    continue;
                }

                n += code_chunk;
            }

            code.processed_code_line = n;
            code.lang_tool = client.get_lang_tool(&code.processed_code_line).await;
            code_lines.push(code);
        }

        return code_lines;
    }

    fn is_code_line(prog_line: &ProgrammingLine) -> bool {
        return match prog_line.prog_type {
            crate::programming_lang::ProgrammingLineType::Code => true,
            crate::programming_lang::ProgrammingLineType::CodeWithComment => true,
            _ => false,
        };
    }
}
