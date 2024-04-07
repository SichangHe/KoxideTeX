use super::*;

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
pub struct Token {
    pub text: String,
    pub loc: Option<SourceLocation>,
    /// don't expand the token
    pub noexpand: Option<bool>,
    /// used in \noexpand
    pub treat_as_relax: Option<bool>,
}

impl Token {
    pub fn new(text: String, loc: Option<SourceLocation>) -> Self {
        Token {
            text,
            loc,
            noexpand: None,
            treat_as_relax: None,
        }
    }
    /// Given a pair of tokens (self and end_token), compute a `Token` encompassing
    /// the whole input range enclosed by these two.
    pub fn range(&self, end_token: &Token, text: String) -> Token {
        Token::new(
            text,
            SourceLocation::range(self.loc.as_ref(), end_token.loc.as_ref()),
        )
    }
}

impl LexerInterface for Token {}

impl LexerInterface for SourceLocation {}

/// Interface required to break circular dependency between Token, Lexer, and
/// ParseError.
pub trait LexerInterface {}
