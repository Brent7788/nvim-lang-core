use std::str::from_utf8_unchecked;

use crate::common::string::DelimiterType;

#[derive(Debug)]
pub enum ProgrammingLanguageType {
    Lua,
    Rust,
}

#[derive(Debug)]
pub enum NamingConvetionType {
    CamelCase,
    PascalCase,
    None,
}

#[derive(Debug)]
pub struct ProgrammingLanguage<const OPERATOR_COUNT: usize, const RESERVED_KEYWORD_COUNT: usize> {
    pub extension: &'static str,
    pub comment_delimiter: &'static str,
    pub block_comment: CodeBlockSyntax,
    pub operators_and_syntax: [&'static str; OPERATOR_COUNT],
    pub reserved_keywords: [&'static str; RESERVED_KEYWORD_COUNT],
    pub string_syntax: [ProgrammingStringSyntax; 2],
    pub block_string: CodeBlockSyntax,
    pub naming_conventions: [NamingConvetionType; 2],
    pub lang_type: ProgrammingLanguageType,
    post_replace: Option<[&'static str; 2]>,
}

pub const LUA: ProgrammingLanguage<27, 21> = ProgrammingLanguage {
    extension: ".lua",
    comment_delimiter: "--",
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
            string_ignore_delimiter: [DelimiterType::DelimiterStr("\\\""), DelimiterType::None],
        },
        ProgrammingStringSyntax {
            string_delimiter: DelimiterType::DelimiterChar('\''),
            string_ignore_delimiter: [DelimiterType::DelimiterStr("\\\'"), DelimiterType::None],
        },
    ],
    reserved_keywords: [
        "and", "break", "do", "else", "elseif", "end", "false", "for", "function", "if", "in",
        "local", "nil", "not", "or", "repeat", "return", "then", "true", "until", "while",
    ],
    operators_and_syntax: [
        "_", "+", "-", "*", "/", "%", "=", "'", "\"", "~", ">", "<", "^", "/=", "%=", "(", ")",
        "[", "]", "{", "}", ";", ":", ",", "..", ".", "#",
    ],
    naming_conventions: [NamingConvetionType::None, NamingConvetionType::None],
    lang_type: ProgrammingLanguageType::Lua,
    post_replace: None,
};

pub const RUST: ProgrammingLanguage<29, 50> = ProgrammingLanguage {
    extension: ".rs",
    comment_delimiter: "//",
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
            DelimiterType::DelimiterStr("r#\""),
            DelimiterType::DelimiterStr("r##\""),
            DelimiterType::DelimiterStr("r###\""),
            DelimiterType::DelimiterStr("r####\""),
        ],
        end_delmiters: [
            DelimiterType::DelimiterStr("\"#"),
            DelimiterType::DelimiterStr("\"##"),
            DelimiterType::DelimiterStr("\"###"),
            DelimiterType::DelimiterStr("\"####"),
        ],
    },
    string_syntax: [
        ProgrammingStringSyntax {
            string_delimiter: DelimiterType::DelimiterChar('"'),
            string_ignore_delimiter: [DelimiterType::DelimiterStr("\\\""), DelimiterType::None],
        },
        ProgrammingStringSyntax {
            string_delimiter: DelimiterType::DelimiterChar('\''),
            string_ignore_delimiter: [DelimiterType::None, DelimiterType::None],
        },
    ],
    reserved_keywords: [
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
        "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
        "mut", "pub", "ref", "return", "Self", "self", "static", "struct", "super", "trait",
        "true", "type", "unsafe", "use", "where", "while", "str", "usize", "isize", "bool", "i8",
        "i16", "i32", "i64", "u8", "u16", "u32", "u64",
    ],
    operators_and_syntax: [
        "_", "+", "-", "*", "/", "%", "=", "\"", "!", ">", "<", "&", "|", "'", "^", "/=", "%=",
        "(", ")", "{", "}", "[", "]", ";", ":", ",", "..", ".", "#",
    ],
    naming_conventions: [NamingConvetionType::PascalCase, NamingConvetionType::None],
    lang_type: ProgrammingLanguageType::Rust,
    post_replace: Some(["&'", "<'"]),
};

impl<const OPERATOR_COUNT: usize, const RESERVED_KEYWORD_COUNT: usize>
    ProgrammingLanguage<OPERATOR_COUNT, RESERVED_KEYWORD_COUNT>
{
    pub fn is_reserved_keyword(&self, input: &str) -> bool {
        for reserved_keyword in &self.reserved_keywords {
            if input == *reserved_keyword {
                return true;
            }
        }

        return false;
    }

    pub fn post_replace_empty_space(&self, mut line: String) -> String {
        return match self.post_replace {
            Some(r) => {
                for c in r {
                    line = line.replace(c, "");
                }

                return line;
            }
            None => line,
        };
    }

    pub fn replase_all_operators_and_syntax_with_whitespace(&self, mut input: String) -> String {
        for op_snt in &self.operators_and_syntax {
            input = input.replace(op_snt, " ");
        }

        return input;
    }

    pub fn replase_all_reserved_keywords_with_whitespace(&self, mut input: String) -> String {
        let mut transform = String::new();
        let split_whitespace = input.split_whitespace();

        'ignore_chunk: for chunk in split_whitespace {
            let chunk = chunk.trim();
            // HACK: This will ignore all one/two char words
            if chunk.len() <= 2 {
                continue;
            }

            for keyword in self.reserved_keywords {
                if keyword == chunk {
                    continue 'ignore_chunk;
                }
            }

            let chunk = self.split_by_uppercase(chunk);
            let chunk = chunk.trim();
            transform.push_str(chunk);
            transform.push(' ');
        }

        return transform;
    }

    // WARN: This might not work on utf16 strings!
    fn split_by_uppercase<'i>(&self, word: &'i str) -> String {
        // TODO: Need to remove this const and put it somewhere else.
        const UPPERCASE_UTF8: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut output = String::new();
        let input_bytes = word.as_bytes();
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
        }

        if output.is_empty() {
            return String::from(word);
        }

        return output;
    }

    pub fn is_start_of_code_block(&self, line: &str) -> CodeBlockType {
        let comment_code_block_line_syntax = self.block_comment.get_code_block_line_syntax(line);
        let string_code_block_line_syntax = self.block_string.get_code_block_line_syntax(line);

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
pub struct ProgrammingStringSyntax {
    pub string_delimiter: DelimiterType,
    pub string_ignore_delimiter: [DelimiterType; 2],
}

#[derive(Debug)]
pub struct CodeBlockSyntax {
    start_delmiters: [DelimiterType; 4],
    end_delmiters: [DelimiterType; 4],
}

#[derive(Debug)]
pub struct CodeBlockLineSyntax {
    pub start_indexof: usize,
    pub start_delimiter: DelimiterType,
    pub end_delimiter: DelimiterType,
}

impl CodeBlockSyntax {
    pub fn get_code_block_line_syntax(&self, value: &str) -> CodeBlockLineSyntax {
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
