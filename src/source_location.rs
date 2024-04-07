use super::*;

/// Lexing or parsing positional information for error reporting.
/// This object is immutable.
#[derive(Clone)]
pub struct SourceLocation {
    /// Lexer holding the input string.
    pub lexer: Arc<dyn LexerInterface>,
    /// Start offset, zero-based inclusive.
    pub start: usize,
    /// End offset, zero-based exclusive.
    pub end: usize,
}

impl SourceLocation {
    pub fn new(lexer: Arc<dyn LexerInterface>, start: usize, end: usize) -> Self {
        SourceLocation { lexer, start, end }
    }

    /// Merges two `SourceLocation`s from location providers, given they are provided in order of appearance.
    ///
    /// - Returns the first one's location if only the first is provided.
    /// - Returns a merged range of the first and the last if both are provided and their lexers match.
    /// - Otherwise, returns `None`.
    pub fn range(
        first: Option<&SourceLocation>,
        second: Option<&SourceLocation>,
    ) -> Option<SourceLocation> {
        match (first, second) {
            (Some(first_loc), None) => Some(*first_loc),
            (Some(first_loc), Some(second_loc)) => {
                if first_loc.lexer as *const _ == second_loc.lexer as *const _ {
                    Some(SourceLocation::new(
                        first_loc.lexer,
                        first_loc.start,
                        second_loc.end,
                    ))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
