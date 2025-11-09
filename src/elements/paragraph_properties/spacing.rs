use std::fmt;

/// Contains before/after spacing, line spacing rule, and line spacing value
#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphSpacing {
    pub before: Option<u32>,
    pub after: Option<u32>,
    pub line: Option<u32>,
    pub line_rule: Option<LineRule>,
    pub before_autospacing: Option<bool>,
    pub after_autospacing: Option<bool>,
}

impl ParagraphSpacing {
    pub fn new(
        before: Option<u32>,
        after: Option<u32>,
        line: Option<u32>,
        line_rule: Option<LineRule>,
        before_autospacing: Option<bool>,
        after_autospacing: Option<bool>,
    ) -> Self {
        Self {
            before,
            after,
            line,
            line_rule,
            before_autospacing,
            after_autospacing,
        }
    }

    pub fn change_before(&mut self, before: Option<u32>) {
        self.before = before;
    }

    pub fn change_after(&mut self, after: Option<u32>) {
        self.after = after;
    }

    pub fn change_line(&mut self, line: Option<u32>) {
        self.line = line;
    }

    pub fn change_line_rule(&mut self, line_rule: Option<LineRule>) {
        self.line_rule = line_rule;
    }

    pub fn change_before_autospacing(&mut self, before_autospacing: Option<bool>) {
        self.before_autospacing = before_autospacing;
    }

    pub fn change_after_autospacing(&mut self, after_autospacing: Option<bool>) {
        self.after_autospacing = after_autospacing;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineRule {
    Auto,
    AtLeast,
    Exact,
}

impl fmt::Display for LineRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineRule::Auto => write!(f, "auto"),
            LineRule::AtLeast => write!(f, "atLeast"),
            LineRule::Exact => write!(f, "exact"),
        }
    }
}
