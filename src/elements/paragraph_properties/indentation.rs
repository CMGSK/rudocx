use crate::errors::RudocxParagraphStyleError;

type Result<T> = std::result::Result<T, RudocxParagraphStyleError>;

/// Contains left, right, first line, and hanging indentation values along with their
/// corresponding additional indentation in hinthedths of character unit
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParagraphIndentation {
    pub left: Option<i32>,
    pub left_chars: Option<i32>,
    pub right: Option<i32>,
    pub right_chars: Option<i32>,
    pub first_line: Option<i32>,
    pub first_line_chars: Option<i32>,
    pub hanging: Option<i32>,
    pub hanging_chars: Option<i32>,
}

impl ParagraphIndentation {
    pub fn new(
        left: Option<i32>,
        left_chars: Option<i32>,
        right: Option<i32>,
        right_chars: Option<i32>,
        first_line: Option<i32>,
        first_line_chars: Option<i32>,
        hanging: Option<i32>,
        hanging_chars: Option<i32>,
    ) -> Result<Self> {
        if hanging.is_some() && first_line.is_some() {
            return Err(RudocxParagraphStyleError::MutuallyExclusive(
                String::from("hanging"),
                String::from("firstLine"),
            ));
        }
        Ok(Self {
            left,
            left_chars,
            right,
            right_chars,
            first_line,
            first_line_chars,
            hanging,
            hanging_chars,
        })
    }

    pub fn change_left(&mut self, left: Option<i32>) {
        self.left = left;
    }

    pub fn change_left_chars(&mut self, left_chars: Option<i32>) {
        self.left_chars = left_chars;
    }

    pub fn change_right(&mut self, right: Option<i32>) {
        self.right = right;
    }

    pub fn change_right_chars(&mut self, right_chars: Option<i32>) {
        self.right_chars = right_chars;
    }

    pub fn change_first_line(&mut self, first_line: Option<i32>) {
        self.first_line = first_line;
    }

    pub fn change_first_line_chars(&mut self, first_line_chars: Option<i32>) {
        self.first_line_chars = first_line_chars;
    }

    pub fn change_hanging(&mut self, hanging: Option<i32>) {
        self.hanging = hanging;
    }

    pub fn change_hanging_chars(&mut self, hanging_chars: Option<i32>) {
        self.hanging_chars = hanging_chars;
    }
}
