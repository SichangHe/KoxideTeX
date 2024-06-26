use std::{
    cmp::PartialEq,
    collections::HashMap,
    fmt::{self, Display},
    sync::Arc,
};

use lazy_regex::regex;
use thiserror::Error;

pub mod build_common;
pub mod build_tree;
pub mod dom_tree;
pub mod macro_expander;
pub mod options;
pub mod parse_error;
pub mod parse_node;
pub mod parse_tree;
pub mod parser;
pub mod settings;
pub mod source_location;
pub mod token;
pub mod utils;

use build_common::*;
use build_tree::*;
use dom_tree::*;
use macro_expander::*;
use options::*;
use parse_error::*;
use parse_node::*;
use parse_tree::*;
use parser::*;
use settings::*;
use source_location::*;
use token::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Parse and build an expression, and return the markup for that.
pub fn render_to_string(expression: &str, options: &Settings) -> String {
    // Invoke the render_to_dom_tree function and convert the resulting DOM tree to markup
    render_to_dom_tree(expression, options).to_markup()
}

/// If the given error is a KaTeX ParseError and options.throwOnError is false,
/// renders the invalid LaTeX as a span with hover title giving the KaTeX
/// error message.  Otherwise, simply throws the error.
fn render_error(error: ParseError, expression: &str, options: &Settings) -> DomSpan {
    let node = make_span(
        Some(vec!["katex-error".into()]),
        Some(vec![SymbolNode {
            text: expression.to_string(),
            ..Default::default()
        }]),
        None,
        None,
    );
    node.set_attribute("title", &error.to_string());
    node.set_attribute("style", &format!("color:{}", options.error_color));
    node
}

/// Generates and returns the KaTeX build tree. This is used for advanced
/// use cases (like rendering to custom output).
pub fn render_to_dom_tree(expression: &str, options: &Settings) -> DomSpan {
    let settings = options;
    match parse_tree(expression, settings) {
        Ok(tree) => build_tree(tree, expression, settings),
        Err(error) => render_error(error, expression, settings),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
