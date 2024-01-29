use log::info;

use crate::{
    lang_tool_client::LangToolClient,
    modules::LangTool,
    programming_lang::{ProgrammingFile, ProgrammingLine},
};

//TODO: Find better name
#[derive(Debug)]
pub struct LangToolCore<'ltc> {
    comments: Vec<Comment<'ltc>>,
}

impl<'ltc> LangToolCore<'ltc> {
    pub async fn new(
        prog_file: &'ltc ProgrammingFile<'ltc>,
        client: &LangToolClient,
    ) -> LangToolCore<'ltc> {
        return LangToolCore {
            comments: Comment::generate(prog_file, client).await,
        };
    }
}

#[derive(Debug)]
struct Comment<'c> {
    prog_lines: Vec<&'c ProgrammingLine>,
    line_end_offset: Vec<usize>,
    comment: String,
    lang_tool: Option<LangTool>,
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
            if !Comment::is_line_comment(prog_line) {
                comment.lang_tool = client.get_lang_tool(&comment.comment).await;
                comments.push(comment);
                comment = Comment::new();
                continue;
            }

            comment.push_line_end_offset(prog_line);

            comment.comment = format!("{}{}", comment.comment.as_str(), prog_line.get_comment());

            comment.prog_lines.push(prog_line);
        }

        if comment.prog_lines.len() > 0 {
            comments.push(comment);
        }

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

        let offset = prog_line.original_line.len() - 1 + last_line_end_offset;

        self.line_end_offset.push(offset);
    }
}
