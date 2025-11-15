use std::fmt;

/// Contains a collection of custom tab stop positions and alignments
#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphTab {
    pub val: ParagraphTabValues,
    pub pos: i32,
    pub leader: Option<ParagraphTabLeaders>,
}

// There's no default implementation since tab val and pos are mandatory to specify on cretaion
impl ParagraphTab {
    pub fn new(val: ParagraphTabValues, pos: i32, leader: Option<ParagraphTabLeaders>) -> Self {
        Self { val, pos, leader }
    }

    pub fn value(&self) -> String {
        self.val.to_string()
    }

    pub fn change_value(&mut self, val: ParagraphTabValues) {
        self.val = val;
    }

    pub fn change_pos(&mut self, pos: i32) {
        self.pos = pos;
    }

    pub fn change_leader(&mut self, leader: Option<ParagraphTabLeaders>) {
        self.leader = leader;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphTabValues {
    Clear,
    Left,
    Center,
    Right,
    Decimal,
    Bar,
    Num,
}

impl fmt::Display for ParagraphTabValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParagraphTabValues::Clear => write!(f, "clear"),
            ParagraphTabValues::Left => write!(f, "left"),
            ParagraphTabValues::Center => write!(f, "center"),
            ParagraphTabValues::Right => write!(f, "right"),
            ParagraphTabValues::Decimal => write!(f, "decimal"),
            ParagraphTabValues::Bar => write!(f, "bar"),
            ParagraphTabValues::Num => write!(f, "num"),
        }
    }
}

impl<T: Into<String>> From<T> for ParagraphTabValues {
    fn from(v: T) -> Self {
        match v.into().as_ref() {
            "clear" => ParagraphTabValues::Clear,
            "left" => ParagraphTabValues::Left,
            "center" => ParagraphTabValues::Center,
            "right" => ParagraphTabValues::Right,
            "decimal" => ParagraphTabValues::Decimal,
            "bar" => ParagraphTabValues::Bar,
            "num" => ParagraphTabValues::Num,
            _ => ParagraphTabValues::Clear,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphTabLeaders {
    None,
    Dot,
    Heavy,
    Hyphen,
    MiddleDot,
}

impl fmt::Display for ParagraphTabLeaders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParagraphTabLeaders::None => write!(f, "none"),
            ParagraphTabLeaders::Dot => write!(f, "dot"),
            ParagraphTabLeaders::Heavy => write!(f, "heavy"),
            ParagraphTabLeaders::Hyphen => write!(f, "hyphen"),
            ParagraphTabLeaders::MiddleDot => write!(f, "middleDot"),
        }
    }
}

impl<T: Into<String>> From<T> for ParagraphTabLeaders {
    fn from(v: T) -> Self {
        match v.into().as_ref() {
            "none" => ParagraphTabLeaders::None,
            "dot" => ParagraphTabLeaders::Dot,
            "heavy" => ParagraphTabLeaders::Heavy,
            "hyphen" => ParagraphTabLeaders::Hyphen,
            "middleDot" => ParagraphTabLeaders::MiddleDot,
            _ => ParagraphTabLeaders::None,
        }
    }
}
