pub mod parse_tree;
pub mod parser;
pub mod settings;
pub mod macro_expander;
pub mod token;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Parse and build an expression, and return the markup for that.
pub fn render_to_string(expression: &str, options: &SettingsOptions) -> String {
    // Invoke the render_to_dom_tree function and convert the resulting DOM tree to markup
    render_to_dom_tree(expression, options).to_markup()
}

/// Generates and returns the KaTeX build tree. This is used for advanced
/// use cases (like rendering to custom output).
pub fn render_to_dom_tree(expression: &str, options: &SettingsOptions) -> DomSpan {
    let settings = Settings::new(options);
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
