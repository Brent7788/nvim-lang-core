#[derive(Clone, Copy)]
pub struct StringPosition {
    pub start_index: usize,
    pub end_index: usize,

    state: StringPositionState,
}

impl StringPosition {
    pub fn empty_positions<const N: usize>() -> [Option<StringPosition>; N] {
        return [None; N];
    }

    fn new_empty() -> Self {
        return Self {
            start_index: 0,
            end_index: 0,
            state: StringPositionState::Empty,
        };
    }

    fn set_start_index_if_char(&mut self, index: usize, str_byte: &u8, delimiter: &DelimiterType) {
        if !matches!(self.state, StringPositionState::Empty) {
            return;
        }

        if delimiter.is_char_not_equal(*str_byte) {
            return;
        }

        self.start_index = index;
        self.state = StringPositionState::StartIndexSetted;
    }

    fn set_start_index_if_str(
        &mut self,
        index: usize,
        str_bytes: &[u8],
        delimiter: &DelimiterType,
    ) {
        if !matches!(self.state, StringPositionState::Empty) {
            return;
        }

        if delimiter.is_not_equal(index, str_bytes) {
            return;
        }

        self.start_index = index;
        self.state = StringPositionState::StartIndexSetted;
    }

    fn set_end_index_if_char(&mut self, index: usize, str_byte: &u8, delimiter: &DelimiterType) {
        if !matches!(self.state, StringPositionState::StartIndexSetted) {
            return;
        }

        if delimiter.is_char_not_equal(*str_byte) {
            return;
        }

        self.end_index = index;
        self.state = StringPositionState::StartAndEndIndexSetted;
    }

    fn set_end_index_if_str(&mut self, index: usize, str_bytes: &[u8], delimiter: &DelimiterType) {
        if !matches!(self.state, StringPositionState::StartIndexSetted) {
            return;
        }

        if delimiter.is_not_equal(index, str_bytes) {
            return;
        }

        self.end_index = index;
        self.state = StringPositionState::StartAndEndIndexSetted;
    }

    fn set_end_index(&mut self, index: usize) {
        if !matches!(self.state, StringPositionState::StartIndexSetted) {
            return;
        }

        self.end_index = index;
        self.state = StringPositionState::StartAndEndIndexSetted;
    }
}

#[derive(Clone, Copy)]
enum StringPositionState {
    Empty,
    StartIndexSetted,
    StartAndEndIndexSetted,
}

pub trait StringPositionTrait<const N: usize> {
    fn push_str_pos(&mut self, current_str_pos: StringPosition);
}

impl<const N: usize> StringPositionTrait<N> for [Option<StringPosition>; N] {
    fn push_str_pos(&mut self, current_str_pos: StringPosition) {
        for str_pos in self {
            if !matches!(str_pos, None) {
                continue;
            }

            *str_pos = Some(current_str_pos);
        }
    }
}

pub enum DelimiterType<'dt> {
    DelimiterStr(&'dt str),
    DelimiterChar(char),
    None,
}

impl<'dt> DelimiterType<'dt> {
    pub fn is_char_not_equal(&self, str_byte: u8) -> bool {
        let delimiter = match self {
            DelimiterType::DelimiterChar(delimiter) => *delimiter as u8,
            _ => return true,
        };

        if str_byte != delimiter {
            return true;
        }

        return false;
    }

    pub fn is_not_equal(&self, index: usize, str_bytes: &[u8]) -> bool {
        return match self {
            DelimiterType::DelimiterStr(dlm_str) => test(index, str_bytes, dlm_str.as_bytes()),
            DelimiterType::DelimiterChar(dlm_char) => str_bytes[index] != (*dlm_char as u8),
            DelimiterType::None => true,
        };
    }
}

//TODO: Change fn name
fn test(index: usize, str_bytes: &[u8], delimiter: &[u8]) -> bool {
    let str_bytes = &str_bytes[index..];

    if str_bytes.len() < delimiter.len() {
        return true;
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

    return !matched;
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

        let mut current_str_pos = StringPosition::new_empty();

        for str_byte in string_bytes {
            current_str_pos.set_start_index_if_char(index, str_byte, delimiter);
            current_str_pos.set_start_index_if_str(index, string_bytes, delimiter);

            //TODO: Need to ignore bytes here

            current_str_pos.set_end_index_if_char(index, str_byte, delimiter);
            current_str_pos.set_end_index_if_str(index, string_bytes, delimiter);
            index += 1;

            if index == string_bytes.len() {
                current_str_pos.set_end_index(index);
            }

            if matches!(
                current_str_pos.state,
                StringPositionState::StartAndEndIndexSetted
            ) {
                string_positions.push_str_pos(current_str_pos);
                current_str_pos = StringPosition::new_empty();
            }
        }

        //TODO: Need to deconstruct string_positions here

        return &[""; S];
    }
}
