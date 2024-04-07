use super::*;

/// This is the ParseError struct, which is the main error type thrown by KaTeX
/// functions when something has gone wrong. This is used to distinguish internal
/// errors from errors in the expression that the user provided.
///
/// If possible, a caller should provide a Token or ParseNode with information
/// about where in the source string the problem occurred.
#[derive(Debug, Error)]
pub struct ParseError {
    position: Option<usize>, // Error start position based on passed-in Token or ParseNode.
    length: Option<usize>,   // Length of affected text based on passed-in Token or ParseNode.
    raw_message: String,     // The underlying error message without any context added.
}

impl ParseError {
    pub fn new(message: &str, token: Option<&Token>) -> Self {
        let mut error = format!("KaTeX parse error: {}", message);
        let (mut start, mut end) = (None, None);

        if let Some(token) = token {
            let loc = &token.loc;
            if loc.start <= loc.end {
                // If we have the input and a position, make the error a bit fancier

                // Get the input
                let input = &loc.lexer.input;

                // Prepend some information
                start = Some(loc.start);
                end = Some(loc.end);
                if let Some(start) = start {
                    if start == input.len() {
                        error += " at end of input: ";
                    } else {
                        error += &format!(" at position {}: ", start + 1);
                    }

                    // Underline token in question using combining underscores
                    let underlined = input[start..end]
                        .chars()
                        .map(|c| {
                            if c != '\n' {
                                format!("{}{}", c, '\u{0332}')
                            } else {
                                c.to_string()
                            }
                        })
                        .collect::<String>();

                    // Extract some context from the input and add it to the error
                    let left = if start > 15 {
                        format!("{}{}", "…", &input[start - 15..start])
                    } else {
                        input[..start].to_string()
                    };
                    let right = if end.unwrap() + 15 < input.len() {
                        format!("{}{}", &input[end.unwrap()..end.unwrap() + 15], "…")
                    } else {
                        input[end.unwrap()..].to_string()
                    };
                    error += &format!("{}{}{}", left, underlined, right);
                }
            }
        }

        ParseError {
            position: start,
            length: end.map(|end| end - start.unwrap()),
            raw_message: message.to_string(),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw_message)
    }
}
