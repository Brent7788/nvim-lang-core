use std::str::from_utf8_unchecked;

use crate::common::string::DelimiterType;

#[derive(Debug)]
pub enum ProgrammingLanguageType {
    Lua,
    Rust,
}

#[derive(Debug)]
pub enum ProgrammingLineType {
    NewLine,
    Code,
    CodeWithComment,
    CodeWithString,
    CodeWithStringWithComment,
    Comment,
    BlockCommentStart,
    BlockComment,
    BlockCommentEnd,
    BlockCommentStartAndEnd,
    Unknown,
}

#[derive(Debug)]
pub enum NamingConvetionType {
    CamelCase,
    PascalCase,
    None,
}

#[derive(Debug)]
pub struct ProgrammingLanguage<'lang> {
    pub extension: &'lang str,
    pub comment_delimiter: &'lang str,
    pub block_comment_delimiter_start: &'lang str,
    pub block_comment_delimiter_end: &'lang str,
    pub block_comment: CodeBlockSyntax,
    pub operators_and_syntax: Vec<&'lang str>,
    pub reserved_keywords: Vec<&'lang str>,
    pub string_syntax: [ProgrammingStringSyntax; 2],
    pub block_string: CodeBlockSyntax,
    pub naming_conventions: [NamingConvetionType; 2],
    pub lang_type: ProgrammingLanguageType,
}

impl<'lang> ProgrammingLanguage<'lang> {
    pub fn init() -> [ProgrammingLanguage<'lang>; 2] {
        return [
            ProgrammingLanguage {
                extension: ".lua",
                comment_delimiter: "--",
                block_comment_delimiter_start: "--[[",
                block_comment_delimiter_end: "--]]",
                block_comment: CodeBlockSyntax {
                    start_delmiters: [
                        DelimiterType::DelimiterStr("--[["),
                        DelimiterType::DelimiterStr("--[=["),
                        DelimiterType::DelimiterStr("--[==["),
                        DelimiterType::DelimiterStr("--[===["),
                    ],
                    end_delmiters: [
                        DelimiterType::DelimiterStr("]]"),
                        DelimiterType::DelimiterStr("]=]"),
                        DelimiterType::DelimiterStr("]==]"),
                        DelimiterType::DelimiterStr("]===]"),
                    ],
                },
                block_string: CodeBlockSyntax {
                    start_delmiters: [
                        DelimiterType::DelimiterStr("[["),
                        DelimiterType::DelimiterStr("[=["),
                        DelimiterType::DelimiterStr("[==["),
                        DelimiterType::DelimiterStr("[===["),
                    ],
                    end_delmiters: [
                        DelimiterType::DelimiterStr("]]"),
                        DelimiterType::DelimiterStr("]=]"),
                        DelimiterType::DelimiterStr("]==]"),
                        DelimiterType::DelimiterStr("]===]"),
                    ],
                },
                string_syntax: [
                    ProgrammingStringSyntax {
                        string_delimiter: DelimiterType::DelimiterChar('"'),
                        string_ignore_delimiter: [
                            DelimiterType::DelimiterStr("\\\""),
                            DelimiterType::None,
                        ],
                    },
                    ProgrammingStringSyntax {
                        string_delimiter: DelimiterType::DelimiterChar('\''),
                        string_ignore_delimiter: [
                            DelimiterType::DelimiterStr("\\\'"),
                            DelimiterType::None,
                        ],
                    },
                ],
                reserved_keywords: vec![
                    "and", "break", "do", "else", "elseif", "end", "false", "for", "function",
                    "if", "in", "local", "nil", "not", "or", "repeat", "return", "then", "true",
                    "until", "while",
                ],
                operators_and_syntax: vec![
                    "_", "+", "-", "*", "/", "%", "=", "'", "\"", "~", ">", "<", "^", "/=", "%=",
                    "(", ")", "[", "]", "{", "}", ";", ":", ",", "..", ".", "#",
                ],
                naming_conventions: [NamingConvetionType::None, NamingConvetionType::None],
                lang_type: ProgrammingLanguageType::Lua,
            },
            ProgrammingLanguage {
                extension: ".rs",
                comment_delimiter: "//",
                block_comment_delimiter_start: "/*",
                block_comment_delimiter_end: "*/",
                block_comment: CodeBlockSyntax {
                    start_delmiters: [
                        DelimiterType::DelimiterStr("/*"),
                        DelimiterType::None,
                        DelimiterType::None,
                        DelimiterType::None,
                    ],
                    end_delmiters: [
                        DelimiterType::DelimiterStr("*/"),
                        DelimiterType::None,
                        DelimiterType::None,
                        DelimiterType::None,
                    ],
                },
                block_string: CodeBlockSyntax {
                    start_delmiters: [
                        DelimiterType::DelimiterStr("r#"),
                        DelimiterType::DelimiterStr("r##"),
                        DelimiterType::DelimiterStr("r###"),
                        DelimiterType::DelimiterStr("r####"),
                    ],
                    end_delmiters: [
                        DelimiterType::DelimiterStr("#"),
                        DelimiterType::DelimiterStr("##"),
                        DelimiterType::DelimiterStr("###"),
                        DelimiterType::DelimiterStr("####"),
                    ],
                },
                string_syntax: [
                    ProgrammingStringSyntax {
                        string_delimiter: DelimiterType::DelimiterChar('"'),
                        string_ignore_delimiter: [
                            DelimiterType::DelimiterStr("\\\""),
                            DelimiterType::None,
                        ],
                    },
                    ProgrammingStringSyntax::default(),
                ],
                reserved_keywords: vec![
                    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else",
                    "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop",
                    "match", "mod", "move", "mut", "pub", "ref", "return", "Self", "self",
                    "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
                    "while", "str", "usize", "isize", "bool", "i8", "i16", "i32", "i64", "u8",
                    "u16", "u32", "u64",
                ],
                operators_and_syntax: vec![
                    "_", "+", "-", "*", "/", "%", "=", "\"", "!", ">", "<", "&", "|", "'", "^",
                    "/=", "%=", "(", ")", "{", "}", "[", "]", ";", ":", ",", "..", ".", "#",
                ],
                naming_conventions: [NamingConvetionType::PascalCase, NamingConvetionType::None],
                lang_type: ProgrammingLanguageType::Rust,
            },
        ];
    }

    pub fn is_reserved_keyword(&self, input: &str) -> bool {
        for reserved_keyword in &self.reserved_keywords {
            if input == *reserved_keyword {
                return true;
            }
        }

        return false;
    }

    pub fn replase_all_operators_and_syntax_with_whitespace(&self, input: &str) -> String {
        let mut transform = String::from(input);

        for op_snt in &self.operators_and_syntax {
            transform = transform.replace(op_snt, " ");
        }

        return transform;
    }

    // WARN: This might not work on utf16 strings!
    pub fn split_by_naming_conventions<'i>(&self, input: &'i str) -> String {
        // TODO: Need to remove this const and put it somewhere else.
        const LOWERCASE_UTF8: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        const UPPERCASE_UTF8: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut output = String::new();
        // let mut output: Vec<&str> = Vec::new();

        let input_bytes = input.as_bytes();
        let mut current_index: usize = 0;
        let mut start_index: usize = 0;
        let mut is_first_uppercase = true;

        for input_byte in input_bytes {
            for up_alp in UPPERCASE_UTF8 {
                if input_byte == up_alp && is_first_uppercase {
                    start_index = current_index;
                    is_first_uppercase = false;
                    break;
                }

                if input_byte == up_alp {
                    let input_byte_slice = &input_bytes[start_index..current_index];
                    let utf8_input = unsafe { from_utf8_unchecked(input_byte_slice) };
                    output.push(' ');
                    output.push_str(utf8_input);
                    // output.push(k);
                    start_index = current_index;
                    break;
                }
            }

            current_index += 1;
        }

        if start_index < current_index {
            let input_byte_slice = &input_bytes[start_index..current_index];
            let utf8_input = unsafe { from_utf8_unchecked(input_byte_slice) };
            output.push(' ');
            output.push_str(utf8_input);
            // output.push(k);
        }

        if output.is_empty() {
            return String::from(input);
            // output.push(input);
        }

        return output;
    }

    pub fn is_start_of_code_block(&self, line: &str) -> CodeBlockType {
        let comment_code_block_line_syntax = self.block_comment.to_code_block_type(line);
        let string_code_block_line_syntax = self.block_string.to_code_block_type(line);

        // TODO: Need to use this index to set normal comment
        let comment_indexof = line.find(self.comment_delimiter).unwrap_or(usize::MAX);

        if comment_code_block_line_syntax.start_indexof
            < string_code_block_line_syntax.start_indexof
            && comment_code_block_line_syntax.start_indexof < comment_indexof
        {
            if !matches!(
                comment_code_block_line_syntax.end_delimiter.indexof(line),
                None
            ) {
                return CodeBlockType::None;
            }

            return CodeBlockType::Comment(comment_code_block_line_syntax);
        }

        if string_code_block_line_syntax.start_indexof
            < comment_code_block_line_syntax.start_indexof
            && string_code_block_line_syntax.start_indexof < comment_indexof
        {
            if !matches!(
                string_code_block_line_syntax.end_delimiter.indexof(line),
                None
            ) {
                return CodeBlockType::None;
            }

            return CodeBlockType::String(string_code_block_line_syntax);
        }

        return CodeBlockType::None;
    }
}

#[derive(Debug, Default)]
struct ProgrammingStringSyntax {
    string_delimiter: DelimiterType,
    string_ignore_delimiter: [DelimiterType; 2],
}

#[derive(Debug)]
pub struct CodeBlockSyntax {
    start_delmiters: [DelimiterType; 4],
    end_delmiters: [DelimiterType; 4],
}

#[derive(Debug)]
pub struct CodeBlockLineSyntax {
    start_indexof: usize,
    pub start_delimiter: DelimiterType,
    pub end_delimiter: DelimiterType,
}

impl CodeBlockSyntax {
    fn to_code_block_type(&self, value: &str) -> CodeBlockLineSyntax {
        let mut index = 0;
        let mut indexof = usize::MAX;
        let mut start_delimiter_type = DelimiterType::None;
        let mut end_delimiter_type = DelimiterType::None;
        for start_delimiter in &self.start_delmiters {
            if indexof != usize::MAX {
                break;
            }

            match start_delimiter {
                DelimiterType::DelimiterStr(s) => {
                    indexof = value.find(s).unwrap_or(indexof);
                    start_delimiter_type = *start_delimiter;
                    end_delimiter_type = self.end_delmiters[index];
                }
                DelimiterType::DelimiterChar(c) => {
                    indexof = value.find(*c).unwrap_or(indexof);
                    start_delimiter_type = *start_delimiter;
                    end_delimiter_type = self.end_delmiters[index];
                }
                DelimiterType::None => break,
            }

            index += 1;
        }

        return CodeBlockLineSyntax {
            start_indexof: indexof,
            start_delimiter: start_delimiter_type,
            end_delimiter: end_delimiter_type,
        };
    }
}

#[derive(Debug)]
pub enum CodeBlockType {
    String(CodeBlockLineSyntax),
    Comment(CodeBlockLineSyntax),
    None,
}
