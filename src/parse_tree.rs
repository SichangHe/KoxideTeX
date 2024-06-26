use super::*;

/// Parses an expression using a Parser, then returns the parsed result.
pub fn parse_tree(to_parse: &str, settings: &Settings) -> Result<Vec<AnyParseNode>, ParseError> {
    let mut parser = Parser::new(to_parse, settings);

    // Blank out any \df@tag to avoid spurious "Duplicate \tag" errors
    parser.gullet.macros.current.remove("\\df@tag");

    let tree = parser.parse()?;

    // Prevent a color definition from persisting between calls to katex.render().
    parser.gullet.macros.current.remove("\\current@color");
    parser.gullet.macros.current.remove("\\color");

    // If the input used \tag, it will set the \df@tag macro to the tag.
    // In this case, we separately parse the tag and wrap the tree.
    if parser.gullet.macros.contains_key("\\df@tag") {
        if !settings.display_mode {
            return Err(ParseError::new(
                "\\tag works only in display equations".into(),
                None,
            ));
        }
        let tree = vec![AnyParseNode::Tag(TagParseNode {
            type_: "tag".into(),
            mode: "text".into(),
            body: tree,
            loc: None,
            tag: parser.subparse(&[Token::new("\\df@tag".into(), None)])?,
        })];
        Ok(tree)
    } else {
        Ok(tree)
    }
}
