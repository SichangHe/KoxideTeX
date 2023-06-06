/// The resulting token returned from `lex`.
///
/// It consists of the token text plus some position information.
/// The position information is essentially a range in an input string,
/// but instead of referencing the bare input string, we refer to the lexer.
/// That way it is possible to attach extra metadata to the input string,
/// like for example a file name or similar.
///
/// The position information is optional, so it is OK to construct synthetic
/// tokens if appropriate. Not providing available position information may
/// lead to degraded error reporting, though.
#[derive(Clone, Debug)]
pub struct Token<'a, L: LexerInterface> {
    pub text: &'a str,
    pub loc: Option<SourceLocation<'a, L>>,
    /// don't expand the token
    pub noexpand: Option<bool>,
    /// used in \noexpand
    pub treat_as_relax: Option<bool>,
}

impl<'a, L: LexerInterface> Token<'a, L> {
    pub fn new(text: &'a str, loc: Option<SourceLocation<'a, L>>) -> Self {
        Token {
            text,
            loc,
            noexpand: None,
            treat_as_relax: None,
        }
    }
    /// Given a pair of tokens (self and end_token), compute a `Token` encompassing
    /// the whole input range enclosed by these two.
    pub fn range(&self, end_token: &Token<'a, L>, text: &'a str) -> Token<'a, L> {
        Token::new(
            text,
            SourceLocation::range(self.loc.as_ref(), end_token.loc.as_ref()),
        )
    }
}

impl<'a, L: LexerInterface> LexerInterface for Token<'a, L> {}

/// Lexing or parsing positional information for error reporting.
/// This object is immutable.
#[derive(Clone, Debug)]
pub struct SourceLocation<'a, L: LexerInterface> {
    /// Lexer holding the input string.
    pub lexer: &'a L,
    /// Start offset, zero-based inclusive.
    pub start: usize,
    /// End offset, zero-based exclusive.
    pub end: usize,
}

impl<'a, L: LexerInterface> SourceLocation<'a, L> {
    pub fn new(lexer: &'a L, start: usize, end: usize) -> Self {
        SourceLocation { lexer, start, end }
    }

    /// Merges two `SourceLocation`s from location providers, given they are provided in order of appearance.
    ///
    /// - Returns the first one's location if only the first is provided.
    /// - Returns a merged range of the first and the last if both are provided and their lexers match.
    /// - Otherwise, returns `None`.
    pub fn range(
        first: Option<&'a SourceLocation<'a, L>>,
        second: Option<&'a SourceLocation<'a, L>>,
    ) -> Option<SourceLocation<'a, L>> {
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

impl<'a, L: LexerInterface> LexerInterface for SourceLocation<'a, L> {}

/// Interface required to break circular dependency between Token, Lexer, and
/// ParseError.
pub trait LexerInterface {}
