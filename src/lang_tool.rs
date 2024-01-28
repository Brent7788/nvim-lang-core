use crate::{modules::LangTool, nvim_lang_core::LangToolClient, programming_lang::ProgrammingLine};

//TODO: Find better name
#[derive(Debug)]
pub struct LangToolCore {}

#[derive(Debug)]
struct Comment<'c> {
    prog_lines: Vec<&'c ProgrammingLine>,
    line_end_offset: Vec<usize>,
    lang_tool: Option<LangTool>,
}

impl<'c> Comment<'c> {
    fn new() -> Comment<'c> {
        return Comment {
            prog_lines: Vec::new(),
            line_end_offset: Vec::new(),
            lang_tool: None,
        };
    }
    fn generate<'pl>(
        prog_lines: &'pl Vec<ProgrammingLine>,
        client: &LangToolClient,
    ) -> Vec<Comment<'pl>> {
        let mut comments: Vec<Comment> = Vec::new();

        let mut comment: Comment = Comment::new();

        for prog_line in prog_lines {
            if !Comment::is_line_comment(prog_line) {
                // TODO: Need to get lang tool by using the client.
                comments.push(comment);
                comment = Comment::new();
                continue;
            }

            comment.line_end_offset.push(prog_line.original_line.len());

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
}

fn get_lang_tool(client: &LangToolClient) -> LangTool {
    unimplemented!();
}
