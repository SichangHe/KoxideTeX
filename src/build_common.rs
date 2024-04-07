use super::*;

/// Makes a span with the given list of classes, list of children, and options.
pub fn make_span(
    classes: Option<Vec<String>>,
    children: Option<Vec<HtmlDomNode>>,
    options: Option<Options>,
    style: Option<CssStyle>,
) -> DomSpan {
    let span = Span::new(classes, children, options, style);

    size_element_from_children(&span);

    span
}
