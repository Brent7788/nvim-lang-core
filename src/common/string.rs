#[derive(Clone, Copy)]
pub struct StringPosition {
    pub start_index: usize,
    pub end_index: Option<usize>,
}

impl StringPosition {
    pub fn empty_positions<const N: usize>() -> [Option<StringPosition>; N] {
        return [None; N];
    }
}

pub trait StringPositionTrait<const N: usize> {
    fn push_str_pos(&mut self, current_str_pos: Option<StringPosition>);
}

impl<const N: usize> StringPositionTrait<N> for [Option<StringPosition>; N] {
    fn push_str_pos(&mut self, current_str_pos: Option<StringPosition>) {
        if current_str_pos.is_not_valid_string_position() {
            return;
        }

        for str_pos in self {
            if !matches!(str_pos, None) {
                continue;
            }

            *str_pos = current_str_pos;
        }
    }
}

trait StringPostionOptionTrait {
    fn set_start_index_if_char(&mut self, index: usize, str_byte: &u8, delimiter: &DelimiterType);
    fn set_start_index_if_str(&mut self, index: usize, str_bytes: &[u8], delimiter: &DelimiterType);
    fn set_end_index_if_char(&mut self, index: usize, str_byte: &u8, delimiter: &DelimiterType);
    fn set_end_index_if_str(&mut self, index: usize, str_bytes: &[u8], delimiter: &DelimiterType);
    fn set_last_end_index(&mut self, index: usize);
    fn is_not_valid_string_position(&self) -> bool;
}

impl StringPostionOptionTrait for Option<StringPosition> {
    fn set_start_index_if_char(&mut self, index: usize, str_byte: &u8, delimiter: &DelimiterType) {
        if !matches!(self, None) {
            return;
        }

        let delimiter = match delimiter {
            DelimiterType::DelimiterChar(delimiter) => *delimiter as u8,
            _ => return,
        };

        if *str_byte == delimiter {
            *self = Some(StringPosition {
                start_index: index,
                end_index: None,
            });
        }
    }

    fn set_start_index_if_str(
        &mut self,
        index: usize,
        str_bytes: &[u8],
        delimiter: &DelimiterType,
    ) {
        if !matches!(self, None) {
            return;
        }

        let delimiter = match delimiter {
            DelimiterType::DelimiterStr(delimiter) => delimiter.as_bytes(),
            _ => return,
        };

        let str_bytes = &str_bytes[index..];

        if str_bytes.len() < delimiter.len() {
            return;
        }

        let mut dlm_index = 0;
        let mut matched = false;

        while dlm_index < delimiter.len() {
            if str_bytes[dlm_index] == delimiter[dlm_index] {
                matched = true;
            } else {
                matched = false;
            }

            dlm_index += 1;
        }

        if matched {
            *self = Some(StringPosition {
                start_index: index,
                end_index: None,
            });
        }
    }

    fn set_end_index_if_char(&mut self, index: usize, str_byte: &u8, delimiter: &DelimiterType) {
        if matches!(self, None) {
            return;
        }

        let delimiter = match delimiter {
            DelimiterType::DelimiterChar(delimiter) => *delimiter as u8,
            _ => return,
        };

        if *str_byte != delimiter {
            return;
        }

        if let Some(str_pos) = self {
            str_pos.end_index = Some(index);
        }
    }

    fn set_end_index_if_str(&mut self, index: usize, str_bytes: &[u8], delimiter: &DelimiterType) {
        todo!()
    }

    fn is_not_valid_string_position(&self) -> bool {
        let str_pot = match self {
            Some(str_pot) => str_pot,
            None => return true,
        };

        if let None = str_pot.end_index {
            return true;
        }

        return false;
    }

    fn set_last_end_index(&mut self, index: usize) {
        let str_pot = match self {
            Some(str_pot) => str_pot,
            None => return,
        };

        if let None = str_pot.end_index {
            return;
        }

        str_pot.end_index = Some(index);
    }
}

pub enum DelimiterType<'dt> {
    DelimiterStr(&'dt str),
    DelimiterChar(char),
    None,
}

pub trait StringDelimiterSlice<const S: usize, const D: usize> {
    fn slices_by(
        &self,
        delimiter: &DelimiterType,
        ignore_by_delimiters: &[DelimiterType; D],
    ) -> &[&str; S];
}

impl<const S: usize, const D: usize> StringDelimiterSlice<S, D> for String {
    fn slices_by(
        &self,
        delimiter: &DelimiterType,
        ignore_by_delimiters: &[DelimiterType; D],
    ) -> &[&str; S] {
        let string_bytes = self.as_bytes();
        let mut index = 0;
        let mut string_positions: [Option<StringPosition>; 8] = StringPosition::empty_positions();

        let mut current_str_pos: Option<StringPosition> = None;
        for str_byte in string_bytes {
            current_str_pos.set_start_index_if_char(index, str_byte, delimiter);
            current_str_pos.set_start_index_if_str(index, string_bytes, delimiter);

            //TODO: Need to ignore bytes here

            current_str_pos.set_end_index_if_char(index, str_byte, delimiter);
            current_str_pos.set_end_index_if_str(index, string_bytes, delimiter);
            index += 1;

            if index == string_bytes.len() {
                current_str_pos.set_last_end_index(index);
            }

            string_positions.push_str_pos(current_str_pos);
        }

        //TODO: Need to deconstruct string_positions here

        return &[""; S];
    }
}
