use crate::errors::RudocxParagraphStyleError;

type Result<T> = std::result::Result<T, RudocxParagraphStyleError>;

/// Contains left, right, first line, and hanging indentation values
#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphIndentation {
    pub left: Option<i32>,
    pub right: Option<i32>,
    pub first_line: Option<i32>,
    pub hanging: Option<u32>,
    pub start: Option<i32>,
    pub end: Option<i32>,
}

impl ParagraphIndentation {
    pub fn new(
        left: Option<i32>,
        right: Option<i32>,
        first_line: Option<i32>,
        hanging: Option<u32>,
        start: Option<i32>,
        end: Option<i32>,
    ) -> Result<Self> {
        if hanging.is_some() && first_line.is_some() {
            return Err(RudocxParagraphStyleError::MutuallyExclusive(
                String::from("hanging"),
                String::from("firstLine"),
            ));
        }
        Ok(Self {
            left,
            right,
            first_line,
            hanging,
            start,
            end,
        })
    }

    pub fn change_left(&mut self, left: Option<i32>) {
        self.left = left;
    }

    pub fn change_right(&mut self, right: Option<i32>) {
        self.right = right;
    }

    pub fn change_first_line(&mut self, first_line: Option<i32>) -> Result<()> {
        if self.hanging.is_some() {
            return Err(RudocxParagraphStyleError::MutuallyExclusive(
                String::from("hanging"),
                String::from("firstLine"),
            ));
        }
        self.first_line = first_line;
        Ok(())
    }

    pub fn change_hanging(&mut self, hanging: Option<u32>) -> Result<()> {
        if self.first_line.is_some() {
            return Err(RudocxParagraphStyleError::MutuallyExclusive(
                String::from("hanging"),
                String::from("firstLine"),
            ));
        }
        self.hanging = hanging;
        Ok(())
    }

    pub fn change_start(&mut self, start: Option<i32>) {
        self.start = start;
    }

    pub fn change_end(&mut self, end: Option<i32>) {
        self.end = end;
    }
}
