use std::{rc::Rc, sync::Arc};

use languagetool_rust::CheckResponse;
use log::error;
use tokio::{spawn, task::JoinHandle};

use crate::{
    code::code_file::{Code, CodeBlock, CodeFile},
    lang_tool_client::LangToolClient,
};

// #[derive(Debug)]
// pub enum LanguageToolLinesType {
//     Comment,
//     Code,
//     String,
//     Undefined,
// }

#[derive(Debug)]
pub enum LanguageToolLineType {
    Block(CodeBlock),
    Code(Vec<Code>),
}

#[derive(Debug)]
pub struct LanguageToolFile {
    // pub code_file: CodeFile,
    pub lines: Vec<LanguageToolLines>,
}

impl LanguageToolFile {
    pub async fn new<const OPERATOR_COUNT: usize, const RESERVED_KEYWORD_COUNT: usize>(
        code_file: CodeFile<'_, OPERATOR_COUNT, RESERVED_KEYWORD_COUNT>,
        client: Arc<LangToolClient>,
    ) -> LanguageToolFile {
        return LanguageToolFile {
            lines: LanguageToolLines::generate(code_file, client).await,
        };
    }
}

#[derive(Debug)]
pub struct LanguageToolLines {
    pub lines: LanguageToolLineType,
    pub lang_tool_response: CheckResponse,
}

impl LanguageToolLines {
    async fn generate<const OPERATOR_COUNT: usize, const RESERVED_KEYWORD_COUNT: usize>(
        code_file: CodeFile<'_, OPERATOR_COUNT, RESERVED_KEYWORD_COUNT>,
        client: Arc<LangToolClient>,
    ) -> Vec<LanguageToolLines> {
        // TODO: At this point I also need to create a cash file and handle it.
        let lang_tool_lines_count = (code_file.blocks.len() + code_file.lines.len()) as usize;
        let mut lang_tool_lines: Vec<LanguageToolLines> = Vec::with_capacity(lang_tool_lines_count);

        let code_block_handle: JoinHandle<Vec<LanguageToolLines>> = spawn(
            LanguageToolLines::code_block_lines(code_file.blocks, client.clone()),
        );

        let code_handle: JoinHandle<Vec<LanguageToolLines>> =
            spawn(LanguageToolLines::code_lines(code_file.lines, client));

        match code_block_handle.await {
            Ok(lines) => {
                lang_tool_lines.extend(lines);
            }
            Err(e) => error!(
                "LanguageToolLines::generate unable to process code block {:?}",
                e
            ),
        };

        match code_handle.await {
            Ok(lines) => {
                lang_tool_lines.extend(lines);
            }
            Err(e) => error!("LanguageToolLines::generate unable to process code {:?}", e),
        };

        return lang_tool_lines;
    }

    async fn code_block_lines(
        blocks: Vec<CodeBlock>,
        client: Arc<LangToolClient>,
    ) -> Vec<LanguageToolLines> {
        let mut lines = Vec::with_capacity(blocks.len());

        for code_block in blocks {
            let lang_tool_response = match client.get_lang_tool(&code_block.block) {
                Some(res) => res,
                None => {
                    error!(
                        "Language Tool Client response is empty. Response Value: {:#?}",
                        code_block
                    );
                    continue;
                }
            };

            lines.push(LanguageToolLines {
                lines: LanguageToolLineType::Block(code_block),
                lang_tool_response,
            });
        }

        return lines;
    }

    async fn code_lines(
        code_lines: Vec<Code>,
        client: Arc<LangToolClient>,
    ) -> Vec<LanguageToolLines> {
        let mut lines = Vec::with_capacity(code_lines.len());
        let CHAR_REQUEST_LIMIT = 1000;
        let mut currnet_char_count = 0;
        let mut code_line_count = code_lines.len();
        let mut code_lines_set: Vec<Code> = Vec::with_capacity(code_line_count);

        for code_line in code_lines {
            currnet_char_count += code_line.value.len();
            code_lines_set.push(code_line);

            if currnet_char_count < CHAR_REQUEST_LIMIT {
                continue;
            }

            let mut requests: Vec<&str> = Vec::with_capacity(code_lines_set.len());

            for code_line_set in &code_lines_set {
                requests.push(&code_line_set.value);
            }

            let lang_tool_response = match client.get_multi_lang_tool(requests) {
                Some(res) => res,
                None => {
                    error!("Language Tool Client response is empty.");
                    continue;
                }
            };
            let len = code_line_count - code_lines_set.len();
            lines.push(LanguageToolLines {
                lines: LanguageToolLineType::Code(code_lines_set),
                lang_tool_response,
            });
            code_lines_set = Vec::with_capacity(len);
            currnet_char_count = 0;
        }

        if code_lines_set.is_empty() {
            return lines;
        }

        let mut requests: Vec<&str> = Vec::with_capacity(code_lines_set.len());

        for code_line_set in &code_lines_set {
            requests.push(&code_line_set.value);
        }

        let lang_tool_response = match client.get_multi_lang_tool(requests) {
            Some(res) => res,
            None => {
                error!("Language Tool Client response is empty.");
                return lines;
            }
        };

        lines.push(LanguageToolLines {
            lines: LanguageToolLineType::Code(code_lines_set),
            lang_tool_response,
        });

        return lines;
    }
}
