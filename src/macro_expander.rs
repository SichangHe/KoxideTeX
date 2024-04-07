// This file contains the “gullet” where macros are expanded
// until only non-macro tokens remain.

use super::*;

/// List of commands that act like macros but aren't defined as a macro,
/// function, or symbol. Used in `is_defined`.
pub const IMPLICIT_COMMANDS: [&str; 4] = ["^", "_", "\\limits", "\\nolimits"];

pub struct MacroExpander<'a> {
    settings: &'a Settings,
    expansion_count: usize,
    lexer: Lexer<'a>,
    macros: Namespace<'a>,
    stack: Vec<Token>,
    mode: Mode,
}

impl<'a> MacroExpander<'a> {
    pub fn new(input: &'a str, settings: &'a Settings, mode: Mode) -> Self {
        let lexer = Lexer::new(input, settings);
        let macros = Namespace::new(&functions::MACROS, settings.macros);
        MacroExpander {
            settings,
            expansion_count: 0,
            lexer,
            macros,
            stack: Vec::new(),
            mode,
        }
    }

    pub fn feed(&mut self, input: &'a str) {
        self.lexer = Lexer::new(input, self.settings);
    }

    pub fn switch_mode(&mut self, new_mode: Mode) {
        self.mode = new_mode;
    }

    pub fn begin_group(&mut self) {
        self.macros.begin_group();
    }

    pub fn end_group(&mut self) {
        self.macros.end_group();
    }

    pub fn end_groups(&mut self) {
        self.macros.end_groups();
    }

    pub fn future(&mut self) -> Token {
        if self.stack.is_empty() {
            self.stack.push(self.lexer.lex());
        }
        self.stack.last().cloned().unwrap()
    }

    pub fn pop_token(&mut self) -> Token {
        self.future(); // ensure non-empty stack
        self.stack.pop().unwrap()
    }

    pub fn push_token(&mut self, token: Token) {
        self.stack.push(token);
    }

    pub fn push_tokens(&mut self, tokens: Vec<Token>) {
        self.stack.extend(tokens);
    }

    pub fn scan_argument(&mut self, is_optional: bool) -> Option<Token> {
        let (start, tokens, end) = if is_optional {
            self.consume_spaces();
            if self.future().text != "[" {
                return None;
            }
            let start = self.pop_token(); // don't include [ in tokens
            self.consume_arg(Some("]"))
        } else {
            self.consume_arg(None)
        };

        self.push_token(Token::new("EOF".into(), end.loc));

        self.push_tokens(tokens);
        Some(start.range(&end, "".into()))
    }

    pub fn consume_spaces(&mut self) {
        loop {
            let token = self.future();
            if token.text == " " {
                self.stack.pop();
            } else {
                break;
            }
        }
    }

    pub fn consume_arg(&mut self, delims: Option<&str>) -> (Token, Vec<Token>, Token) {
        let mut tokens = Vec::new();
        let start = self.future();
        let (mut depth, mut match_delim) = (0, false);
        let (mut start_delim, mut end_delim) = (0, 0);
        let delims = delims.map(|d| d.chars().collect::<String>());
        loop {
            let tok = self.pop_token();
            tokens.push(tok.clone());
            if let Some(ref d) = delims {
                if (depth == 0 || (depth == 1 && d[start_delim..start_delim + 1] == *"{"))
                    && tok.text == d[start_delim..start_delim + 1]
                {
                    if !match_delim {
                        start_delim += 1;
                        match_delim = true;
                    }
                    if start_delim == d.len() {
                        tokens.truncate(tokens.len() - start_delim);
                        break;
                    }
                } else if match_delim {
                    start_delim = 0;
                    match_delim = false;
                }
            }
            if tok.text == "{" {
                depth += 1;
            } else if tok.text == "}" {
                depth -= 1;
                if depth == -1 {
                    panic!("Extra }");
                }
            } else if tok.text == "EOF" {
                panic!("Unexpected end of input in a macro argument");
            }
        }

        if start.text == "{" && tokens.last().unwrap().text == "}" {
            tokens.pop();
            tokens.remove(0);
        }

        tokens.reverse();
        (start, tokens, self.pop_token())
    }

    pub fn consume_args(
        &mut self,
        num_args: usize,
        delimiters: Option<Vec<Vec<&str>>>,
    ) -> Vec<Vec<Token>> {
        if let Some(delims) = delimiters {
            if delims.len() != num_args + 1 {
                panic!("The length of delimiters doesn't match the number of args!");
            }
            let delims = delims.iter().map(|d| d[0]).collect::<Vec<_>>();
            for d in &delims {
                let tok = self.pop_token();
                if d != &tok.text {
                    panic!("Use of the macro doesn't match its definition");
                }
            }
        }

        let mut args = Vec::new();
        for i in 0..num_args {
            args.push(
                self.consume_arg(delimiters.as_ref().and_then(|d| d.get(i + 1)).map(|d| d[0]))
                    .1,
            );
        }
        args
    }

    pub fn count_expansion(&mut self, amount: usize) {
        self.expansion_count += amount;
        if self.expansion_count > self.settings.max_expand {
            panic!("Too many expansions: infinite loop or need to increase maxExpand setting");
        }
    }

    pub fn expand_once(&mut self, expandable_only: bool) -> Option<usize> {
        let top_token = self.pop_token();
        let name = top_token.text.clone();
        let expansion = if !top_token.no_expand {
            self._get_expansion(&name)
        } else {
            None
        };
        match expansion {
            None => {
                if expandable_only && !self.is_defined(&name) {
                    panic!("Undefined control sequence: {}", name);
                }
                self.push_token(top_token);
                None
            }
            Some(exp) => {
                self.count_expansion(1);
                let mut tokens = exp.tokens.clone();
                let args = self.consume_args(exp.num_args, exp.delimiters.clone());
                if exp.num_args > 0 {
                    tokens = tokens.into_iter().rev().collect(); // make a shallow copy
                    for i in (0..tokens.len()).rev() {
                        let tok = &tokens[i];
                        if tok.text == "#" {
                            if i == 0 {
                                panic!("Incomplete placeholder at end of macro body");
                            }
                            let tok = &tokens[i - 1];
                            if tok.text == "#" {
                                tokens.remove(i);
                            } else if let Some(idx) =
                                tok.text.chars().nth(0).and_then(|c| c.to_digit(10))
                            {
                                tokens.splice(i - 1..=i, args[idx as usize - 1].iter().cloned());
                            } else {
                                panic!("Not a valid argument number");
                            }
                        }
                    }
                }
                self.push_tokens(tokens);
                Some(tokens.len())
            }
        }
    }

    pub fn expand_after_future(&mut self) -> Token {
        self.expand_once(false);
        self.future()
    }

    pub fn expand_next_token(&mut self) -> Token {
        loop {
            if let Some(_) = self.expand_once(false) {
                let token = self.stack.pop().unwrap();
                if token.treat_as_relax {
                    token.text = "\\relax".to_string();
                }
                return token;
            }
        }
    }

    pub fn expand_macro(&mut self, name: &str) -> Option<Vec<Token>> {
        self.macros
            .has(name)
            .map(|_| self.expand_tokens(vec![Token::new(name, name)]))
    }

    pub fn expand_tokens(&mut self, mut tokens: Vec<Token>) -> Vec<Token> {
        let mut output = Vec::new();
        let old_stack_length = self.stack.len();
        self.push_tokens(tokens);
        while self.stack.len() > old_stack_length {
            if let Some(_) = self.expand_once(true) {
                let token = self.stack.pop().unwrap();
                if token.treat_as_relax {
                    token.no_expand = false;
                    token.treat_as_relax = false;
                }
                output.push(token);
            }
        }
        self.count_expansion(output.len());
        output
    }

    pub fn expand_macro_as_text(&mut self, name: &str) -> Option<String> {
        self.expand_macro(name)
            .map(|tokens| tokens.iter().map(|token| &token.text).collect::<String>())
    }

    fn _get_expansion(&self, name: &str) -> Option<MacroExpansion> {
        let definition = self.macros.get(name)?;
        if name.len() == 1 {
            if let Some(catcode) = self.lexer.catcodes.get(&name.chars().next().unwrap()) {
                if catcode != &13 {
                    return None;
                }
            }
        }
        if let Some(exp) = match definition {
            &functions::MACROS => functions::macos(name),
            _ => None,
        } {
            if let Some(ref exp) = exp {
                if let Some(idx) = exp.find('#') {
                    let num_args = exp.chars().filter(|&c| c == '#').count();
                    let mut tokens = Vec::new();
                    let mut body_lexer = Lexer::new(exp, self.settings);
                    loop {
                        let tok = body_lexer.lex();
                        if tok.text == "EOF" {
                            break;
                        }
                        tokens.push(tok);
                    }
                    tokens.reverse();
                    return Some(MacroExpansion { tokens, num_args });
                }
            }
        }
        None
    }

    pub fn is_defined(&self, name: &str) -> bool {
        self.macros.has(name)
            || functions::FUNCTIONS.contains_key(name)
            || symbols::MATH_SYMBOLS.contains_key(name)
            || symbols::TEXT_SYMBOLS.contains_key(name)
            || IMPLICIT_COMMANDS.contains(&name)
    }

    pub fn is_expandable(&self, name: &str) -> bool {
        let macro_ = self.macros.get(name);
        if let Some(macro_) = macro_ {
            match macro_ {
                &functions::MACROS => true,
                _ => false,
            }
        } else {
            functions::FUNCTIONS.contains_key(name) && !functions::FUNCTIONS[name].primitive
        }
    }
}

/// Represents a macro expansion.
#[derive(Clone, Debug)]
pub struct MacroExpansion {
    tokens: Vec<Token>,
    num_args: usize,
}
