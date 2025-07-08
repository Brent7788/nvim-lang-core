use std::sync::Arc;

use languagetool_rust::CheckResponse;
use log::{debug, error, info, warn};
use tokio::{spawn, task::JoinHandle};

use crate::{
    code::code_file::{Code, CodeBlock, CodeFile, CodeType},
    lang_tool_client::LangToolClient,
    nvim_language::line::NvimLangLineType,
};

#[derive(Debug)]
pub enum LanguageToolLineType {
    Block(CodeBlock),
    Code(Code),
}

#[derive(Debug)]
pub struct LanguageToolFile {
    pub lines: Vec<LanguageToolLines>,
}

impl LanguageToolFile {
    pub async fn new(code_file: CodeFile, client: Arc<LangToolClient>) -> LanguageToolFile {
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
    async fn generate(code_file: CodeFile, client: Arc<LangToolClient>) -> Vec<LanguageToolLines> {
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
        if blocks.is_empty() {
            return Vec::new();
        }

        let mut lines = Vec::with_capacity(blocks.len());

        for code_block in blocks {
            let lang_tool_response = match client.get_lang_tool_v2(&code_block.block).await {
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

        // debug!("{:#?}", lines);

        return lines;
    }

    async fn code_lines(
        code_lines: Vec<Code>,
        client: Arc<LangToolClient>,
    ) -> Vec<LanguageToolLines> {
        if code_lines.is_empty() {
            return Vec::new();
        }

        let mut lines = Vec::with_capacity(code_lines.len());
        let mut handle_check_response: Vec<JoinHandle<Option<LanguageToolLines>>> = Vec::new();

        for mut code_line in code_lines {
            let client = client.clone();
            handle_check_response.push(spawn(async move {
                if let CodeType::Code = code_line.tp {
                    // TODO: The word 'Ignore', get the len and minus it form the match.offset
                    code_line.value = format!("Ignore {}", code_line.value);
                }

                let mut lang_tool_response = match client.get_lang_tool_v2(&code_line.value).await {
                    Some(res) => res,
                    None => {
                        warn!(
                            "Language Tool Client response is empty. Request Value: {:#?}",
                            code_line
                        );
                        return None;
                    }
                };

                lang_tool_response = match lang_tool_response
                    .handle_repetition(&mut code_line, &client)
                    .await
                {
                    Some(res) => res,
                    None => {
                        error!(
                            "Language Tool Client response is empty. Request Value: {:#?}",
                            code_line
                        );
                        return None;
                    }
                };

                lang_tool_response = lang_tool_response.add_to_offset(&code_line);

                return Some(LanguageToolLines {
                    lines: LanguageToolLineType::Code(code_line),
                    lang_tool_response,
                });
            }));
        }

        for line in handle_check_response {
            match line.await {
                Ok(line) => match line {
                    Some(line) => lines.push(line),
                    None => {}
                },
                Err(e) => error!("LanguageToolLines::code_lines handle error: {:?}", e),
            }
        }

        return lines;
    }

    // TODO: Remove
    //
    // async fn code_lines(
    //     code_lines: Vec<Code>,
    //     client: Arc<LangToolClient>,
    // ) -> Vec<LanguageToolLines> {
    //     if code_lines.is_empty() {
    //         return Vec::new();
    //     }
    //
    //     let mut lines = Vec::with_capacity(code_lines.len());
    //     const CHAR_REQUEST_LIMIT: usize = 1000;
    //     let mut currnet_char_count = 0;
    //     let code_line_count = code_lines.len();
    //     let mut code_lines_set: Vec<Code> = Vec::with_capacity(code_line_count);
    //
    //     for code_line in code_lines {
    //         currnet_char_count += code_line.value.len();
    //         code_lines_set.push(code_line);
    //
    //         if currnet_char_count < CHAR_REQUEST_LIMIT {
    //             continue;
    //         }
    //
    //         let mut requests: Vec<&str> = Vec::with_capacity(code_lines_set.len());
    //
    //         for code_line_set in &mut code_lines_set {
    //             if matches!(code_line_set.tp, CodeType::Code) {
    //                 code_line_set.value = format!("Ignore {}", code_line_set.value);
    //             }
    //
    //             requests.push(&code_line_set.value);
    //         }
    //
    //         let lang_tool_response = match client.get_multi_lang_tool_v2(requests).await {
    //             Some(res) => res,
    //             None => {
    //                 error!("Language Tool Client response is empty.");
    //                 continue;
    //             }
    //         };
    //         let len = code_line_count - code_lines_set.len();
    //         lines.push(LanguageToolLines {
    //             lines: LanguageToolLineType::Code(code_lines_set),
    //             lang_tool_response,
    //         });
    //         code_lines_set = Vec::with_capacity(len);
    //         currnet_char_count = 0;
    //     }
    //
    //     if code_lines_set.is_empty() {
    //         return lines;
    //     }
    //
    //     let mut requests: Vec<&str> = Vec::with_capacity(code_lines_set.len());
    //
    //     for code_line_set in &mut code_lines_set {
    //         if matches!(code_line_set.tp, CodeType::Code) {
    //             code_line_set.value = format!("Ignore {}", code_line_set.value);
    //         }
    //         requests.push(&code_line_set.value);
    //     }
    //
    //     // info!("Request {:#?}", requests);
    //     let lang_tool_response = match client.get_multi_lang_tool_v2(requests).await {
    //         Some(res) => res,
    //         None => {
    //             error!("Language Tool Client response is empty.");
    //             return lines;
    //         }
    //     };
    //
    //     lines.push(LanguageToolLines {
    //         lines: LanguageToolLineType::Code(code_lines_set),
    //         lang_tool_response,
    //     });
    //
    //     return lines;
    // }
}

trait CheckResponseTrait: Sized {
    fn add_to_offset(self, code_line: &Code) -> Self;
    async fn handle_repetition(
        self,
        code_line: &mut Code,
        client: &Arc<LangToolClient>,
    ) -> Option<Self>;
}

impl CheckResponseTrait for CheckResponse {
    // INFO: So Language Tool API does not spell mustakce if there is repetedif word.
    // By adding a comma it will ge ignored.
    async fn handle_repetition(
        self,
        code_line: &mut Code,
        client: &Arc<LangToolClient>,
    ) -> Option<Self> {
        if !matches!(code_line.tp, CodeType::Code) {
            return Some(self);
        }

        let mut re_generate = false;
        for lang_match in &self.matches {
            if lang_match.short_message != "Word repetition" {
                continue;
            }

            re_generate = true;

            let length = match lang_match.replacements.first() {
                Some(r) => r.value.len(),
                None => {
                    error!("Unable to ignore word repetition. Match: {:#?}", lang_match);
                    continue;
                }
            };
            let offset = lang_match.offset + length;
            let new_value = format!(
                "{},{}",
                &code_line.value[..offset],
                &code_line.value[offset..]
            );
            code_line.value = new_value;
        }

        if re_generate {
            return client.get_lang_tool_v2(&code_line.value).await;
        }

        return Some(self);
    }

    // INFO: So what is this. When Code is of type code then we will append 'Ignore '
    // on the value of code. 'Ignore ' have a length of 7, the offset will be used later on the
    // oraginal line.
    fn add_to_offset(mut self, code_line: &Code) -> Self {
        if !matches!(code_line.tp, CodeType::Code) {
            return self;
        }

        for lang_match in &mut self.matches {
            lang_match.offset -= 7;
        }

        return self;
    }
}
