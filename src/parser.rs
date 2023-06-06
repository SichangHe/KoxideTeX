//! This file contains the parser used to parse out a TeX expression from the
//! input. Since TeX isn't context-free, standard parsers don't work particularly
//! well.
//!
//! The strategy of this parser is as such:
//!
//! The main functions (the `.parse...` ones) take a position in the current
//! parse string to parse tokens from. The lexer (found in Lexer.js, stored at
//! this.gullet.lexer) also supports pulling out tokens at arbitrary places. When
//! individual tokens are needed at a position, the lexer is called to pull out a
//! token, which is then used.
//!
//! The parser has a property called "mode" indicating the mode that
//! the parser is currently in. Currently it has to be one of "math" or
//! "text", which denotes whether the current environment is a math-y
//! one or a text-y one (e.g. inside \text). Currently, this serves to
//! limit the functions which can be used in text mode.
//!
//! The main functions then return an object which contains the useful data that
//! was parsed at its given point, and a new position at the end of the parsed
//! data. The main functions can call each other and continue the parsing by
//! using the returned position as a new starting point.
//!
//! There are also extra `.handle...` functions, which pull out some reused
//! functionality into self-contained functions.
//!
//! The functions return ParseNodes.

use crate::{
    settings::Settings,
    token::{LexerInterface, Token},
};

const END_OF_EXPRESSION: &[&str] = &["}", "\\endgroup", "\\end", "\\right", "&"];

pub struct Parser<'a, L: LexerInterface> {
    mode: Mode,
    gullet: MacroExpander,
    settings: &'a Settings,
    leftright_depth: usize,
    next_token: Option<Token<'a, L>>,
}

impl<'a, L: LexerInterface> Parser<'a, L> {
    pub fn new(input: &'a str, settings: &'a Settings) -> Parser<'a, L> {
        Self {
            mode: Mode::Math,
            gullet: MacroExpander::new(input, settings, Mode::Math),
            settings,
            leftright_depth: 0,
            next_token: None,
        }
    }
    /// Checks a result to make sure it has the right type, and throws an
    /// appropriate error otherwise.
    fn expect(&mut self, text: &str, consume: bool) -> Result<(), ParseError<'a, L>> {
        if let Some(next_token) = self.fetch() {
            if next_token.text != text {
                return Err(ParseError::new(
                    format!("Expected '{}', got '{}'", text, next_token.text),
                    next_token,
                ));
            }
            if consume {
                self.consume();
            }
            Ok(())
        } else {
            Err(ParseError::new("Unexpected end of input", None))
        }
    }

    /// Discards the current lookahead token, considering it consumed.
    fn consume(&mut self) {
        self.next_token = None;
    }

    /// Return the current lookahead token, or if there isn't one (at the
    /// beginning, or if the previous lookahead token was consume()d),
    /// fetch the next token as the new lookahead token and return it.
    fn fetch(&mut self) -> Option<&Token<'a, L>> {
        if self.next_token.is_none() {
            self.next_token = self.gullet.expand_next_token();
        }
        self.next_token.as_ref()
    }

    /// Switches between "text" and "math" modes.
    fn switch_mode(&mut self, new_mode: Mode) {
        self.mode = new_mode;
    }

    /// Main parsing function, which parses an entire input.
    pub fn parse(&mut self) -> Result<Vec<AnyParseNode>, ParseError<'a, L>> {
        if !self.settings.global_group {
            // Create a group namespace for the math expression.
            // (LaTeX creates a new group for every $...$, $$...$$, \[...\].)
            self.gullet.begin_group();
        }

        // Use old \color behavior (same as LaTeX's \textcolor) if requested.
        // We do this within the group for the math expression, so it doesn't
        // pollute settings.macros.
        if self.settings.color_is_text_color {
            self.gullet.macros.insert("\\color", "\\textcolor");
        }

        // Try to parse the input
        let parse = self.parse_expression(false);

        if parse.is_ok() {
            // If we succeeded, make sure there's an EOF at the end
            self.expect("EOF", true)?;

            // End the group namespace for the expression
            if !self.settings.global_group {
                self.gullet.end_group();
            }
        }

        // Close any leftover groups in case of a parse error.
        self.gullet.endGroups();

        Ok(parse?)
    }

    /// Fully parse a separate sequence of tokens as a separate job.
    /// Tokens should be specified in reverse order, as in a MacroDefinition.
    fn subparse(
        &mut self,
        tokens: &[Token<'a, L>],
    ) -> Result<Vec<AnyParseNode>, ParseError<'a, L>> {
        // Save the next token from the current job.
        let old_token = self.next_token.take();
        self.consume();

        // Run the new job, terminating it with an excess '}'
        self.gullet.push_token(Token::new("}", None));
        self.gullet.push_tokens(tokens);
        let parse = self.parse_expression(false)?;
        self.expect("}", true)?;

        // Restore the next token from the current job.
        self.next_token = old_token;

        Ok(parse)
    }

    /// Parses an "expression", which is a list of atoms.
    ///
    /// `break_on_infix`: Should the parsing stop when we hit infix nodes? This
    ///                   happens when functions have higher precedence than infix
    ///                   nodes in implicit parses.
    ///
    /// `break_on_token_text`: The text of the token that the expression should end
    ///                        with, or `None` if something else should end the
    ///                        expression.
    fn parse_expression(
        &mut self,
        break_on_infix: bool,
        break_on_token_text: Option<&str>,
    ) -> Result<Vec<AnyParseNode>, ParseError<'a, L>> {
        let mut body = Vec::new();

        // Keep adding atoms to the body until we can't parse any more atoms (either
        // we reached the end, a }, or a \right)
        loop {
            // Ignore spaces in math mode
            if let Mode::Math = self.mode {
                self.consume_spaces();
            }

            let lex = self
                .fetch()
                .ok_or(ParseError::new("Unexpected end of input", None))?;

            if END_OF_EXPRESSION.contains(&lex.text) {
                break;
            }

            if let Some(break_text) = break_on_token_text {
                if lex.text == break_text {
                    break;
                }
            }

            // if break_on_infix && functions.get(lex.text).map_or(false, |func| func.infix) {
            //     break;
            // } // TODO: functions.

            let atom = self.parse_atom(break_on_token_text)?;

            if let Some(atom) = atom {
                if atom.type_ == "internal" {
                    continue;
                }
                body.push(atom);
            } else {
                break;
            }
        }

        if let Mode::Text = self.mode {
            self.form_ligatures(&mut body);
        }

        Ok(self.handle_infix_nodes(body)?)
    }

    /// Rewrites infix operators such as \over with corresponding commands such
    /// as \frac.
    ///
    /// There can only be one infix operator per group. If there's more than one
    /// then the expression is ambiguous. This can be resolved by adding {}.
    fn handle_infix_nodes(
        &self,
        body: Vec<AnyParseNode>,
    ) -> Result<Vec<AnyParseNode>, ParseError<'a, L>> {
        let mut over_index = -1;
        let mut func_name = "";

        for (i, node) in body.iter().enumerate() {
            if let AnyParseNode::Infix(infix_node) = node {
                if over_index != -1 {
                    return Err(ParseError::new(
                        "only one infix operator per group",
                        infix_node.token,
                    ));
                }
                over_index = i;
                func_name = infix_node.replace_with.as_str();
            }
        }

        if over_index != -1 && !func_name.is_empty() {
            let numer_node;
            let denom_node;

            let numer_body = body[..over_index].to_vec();
            let denom_body = body[over_index + 1..].to_vec();

            if numer_body.len() == 1 && numer_body[0].type_ == "ordgroup" {
                numer_node = numer_body[0].clone();
            } else {
                numer_node = AnyParseNode::OrdGroup(OrdGroupNode {
                    mode: self.mode,
                    body: numer_body,
                });
            }

            if denom_body.len() == 1 && denom_body[0].type_ == "ordgroup" {
                denom_node = denom_body[0].clone();
            } else {
                denom_node = AnyParseNode::OrdGroup(OrdGroupNode {
                    mode: self.mode,
                    body: denom_body,
                });
            }

            let node = if func_name == "\\\\abovefrac" {
                self.call_function(
                    func_name,
                    vec![numer_node, body[over_index].clone(), denom_node],
                    vec![],
                )?
            } else {
                self.call_function(func_name, vec![numer_node, denom_node], vec![])?
            };

            Ok(vec![node])
        } else {
            Ok(body)
        }
    }
}

#[derive(Debug)]
pub enum Mode {
    Math,
    Text,
}
