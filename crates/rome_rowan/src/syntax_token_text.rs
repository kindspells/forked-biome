use crate::GreenToken;
use std::fmt::Formatter;
use std::ops::Deref;
use text_size::{TextRange, TextSize};

/// Reference to the text of a SyntaxToken without having to worry about the lifetime of `&str`.
#[derive(Eq, Clone)]
pub struct SyntaxTokenText {
    // Using a green token to ensure this type is Send + Sync.
    token: GreenToken,
    /// Relative range of the "selected" token text.
    range: TextRange,
}

impl SyntaxTokenText {
    pub(crate) fn new(token: GreenToken) -> SyntaxTokenText {
        let range = TextRange::at(TextSize::default(), token.text_len());
        Self { token, range }
    }

    /// Returns the length of the text
    pub fn len(&self) -> TextSize {
        self.range.len()
    }

    /// Returns `true` if the text is empty
    pub fn is_empty(&self) -> bool {
        self.range.is_empty()
    }

    /// Returns a subslice of the text.
    pub fn slice(mut self, range: TextRange) -> SyntaxTokenText {
        assert!(
            self.range.contains_range(range),
            "Range {range:?} exceeds bounds {:?}",
            self.range
        );

        self.range = range;
        self
    }
}

impl Deref for SyntaxTokenText {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.token.text()[self.range]
    }
}

impl std::fmt::Display for SyntaxTokenText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self)
    }
}

impl std::fmt::Debug for SyntaxTokenText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self)
    }
}
impl PartialEq for SyntaxTokenText {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl PartialEq<&'_ str> for SyntaxTokenText {
    fn eq(&self, rhs: &&'_ str) -> bool {
        **self == **rhs
    }
}

impl PartialEq<SyntaxTokenText> for &'_ str {
    fn eq(&self, other: &SyntaxTokenText) -> bool {
        **self == **other
    }
}