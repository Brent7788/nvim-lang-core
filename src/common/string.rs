use log::{debug, info};

#[derive(Debug, Clone, Copy)]
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

    fn try_set_start_index(
        &mut self,
        mut index: usize,
        str_bytes: &[u8],
        delimiter: &DelimiterType,
    ) {
        if !matches!(self.state, StringPositionState::Empty) {
            return;
        }

        if delimiter != (index, str_bytes) {
            return;
        }

        if let DelimiterType::DelimiterStr(dlm_str) = delimiter {
            index = index + dlm_str.len() - 1;
        }

        self.start_index = index + 1;
        self.state = StringPositionState::StartIndexSetted;
    }

    fn try_set_end_index(&mut self, index: usize, str_bytes: &[u8], delimiter: &DelimiterType) {
        if !matches!(self.state, StringPositionState::SettingEndIndex) {
            return;
        }

        if delimiter != (index, str_bytes) {
            return;
        }

        self.end_index = index;
        self.state = StringPositionState::StartAndEndIndexSetted;
    }

    fn set_end_index(&mut self, index: usize) {
        if !matches!(self.state, StringPositionState::SettingEndIndex) {
            return;
        }

        self.end_index = index;
        self.state = StringPositionState::StartAndEndIndexSetted;
    }
}

#[derive(Debug, Clone, Copy)]
enum StringPositionState {
    Empty,
    StartIndexSetted,
    SettingEndIndex,
    StartAndEndIndexSetted,
}

pub trait StringPositionTrait<const N: usize> {
    fn push_str_pos(&mut self, current_str_pos: StringPosition);
    fn as_vec_str(self, s: &String) -> [Option<&str>; N];
}

impl<const N: usize> StringPositionTrait<N> for [Option<StringPosition>; N] {
    fn push_str_pos(&mut self, current_str_pos: StringPosition) {
        for str_pos in self {
            if !matches!(str_pos, None) {
                continue;
            }

            *str_pos = Some(current_str_pos);
            break;
        }
    }

    fn as_vec_str(self, s: &String) -> [Option<&str>; N] {
        let mut strs: [Option<&str>; N] = [None; N];
        let mut index = 0;

        while index < self.len() {
            if let Some(str_pos) = self[index] {
                strs[index] = Some(&s[str_pos.start_index..str_pos.end_index]);
            } else {
                break;
            }

            index += 1;
        }

        return strs;
    }
}

#[derive(Debug)]
pub enum DelimiterType<'dt> {
    DelimiterStr(&'dt str),
    DelimiterChar(char),
    None,
}

impl<'dt> PartialEq<(usize, &[u8])> for &DelimiterType<'dt> {
    fn eq(&self, other: &(usize, &[u8])) -> bool {
        self.is_equal(other.0, other.1)
    }

    fn ne(&self, other: &(usize, &[u8])) -> bool {
        !self.eq(other)
    }
}

impl<'dt> DelimiterType<'dt> {
    fn is_equal(&self, index: usize, str_bytes: &[u8]) -> bool {
        return match self {
            DelimiterType::DelimiterStr(dlm_str) => {
                DelimiterType::is_dlm_str_equal(index, str_bytes, dlm_str.as_bytes())
            }
            DelimiterType::DelimiterChar(dlm_char) => str_bytes[index] == (*dlm_char as u8),
            DelimiterType::None => false,
        };
    }

    fn is_dlm_str_equal(index: usize, str_bytes: &[u8], delimiter: &[u8]) -> bool {
        let str_bytes = &str_bytes[index..];

        if str_bytes.len() < delimiter.len() {
            return false;
        }

        let mut dlm_index = 0;
        let mut matched = false;

        while dlm_index < delimiter.len() {
            if str_bytes[dlm_index] == delimiter[dlm_index] {
                matched = true;
            } else {
                matched = false;
                break;
            }

            dlm_index += 1;
        }

        return matched;
    }
}

pub trait StringDelimiterSlice<const S: usize, const D: usize> {
    fn slices_by(
        &self,
        delimiter: &DelimiterType,
        ignore_by_delimiters: &[DelimiterType; D],
    ) -> [Option<&str>; S];
}

impl<const S: usize, const D: usize> StringDelimiterSlice<S, D> for String {
    fn slices_by(
        &self,
        delimiter: &DelimiterType,
        ignore_by_delimiters: &[DelimiterType; D],
    ) -> [Option<&str>; S] {
        let string_bytes = self.as_bytes();
        let mut index = 0;
        let mut string_positions: [Option<StringPosition>; S] = StringPosition::empty_positions();

        let mut current_str_pos = StringPosition::new_empty();

        'string_bytes_loop: while index < string_bytes.len() {
            // TODO: Put ignore part into a function/method
            for dlm in ignore_by_delimiters {
                if dlm == (index, string_bytes) {
                    if let DelimiterType::DelimiterStr(dlm_str) = dlm {
                        index = index + dlm_str.len() - 1;
                    }

                    index += 1;
                    continue 'string_bytes_loop;
                }
            }

            current_str_pos.try_set_start_index(index, string_bytes, delimiter);
            current_str_pos.try_set_end_index(index, string_bytes, delimiter);

            index += 1;

            if matches!(current_str_pos.state, StringPositionState::StartIndexSetted) {
                current_str_pos.state = StringPositionState::SettingEndIndex;
            }

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

        return string_positions.as_vec_str(self);
    }
}
