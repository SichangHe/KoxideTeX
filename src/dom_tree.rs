//! These objects store the data about the DOM nodes we create, as well as some
//! extra data. They can then be transformed into real DOM nodes with the
//! `to_node` function or HTML markup using `to_markup`. They are useful for both
//! storing extra properties on the nodes, as well as providing a way to easily
//! work with the DOM.
//!
//! Similar functions for working with MathML nodes exist in mathMLTree.js.
//!
//! TODO: refactor `span` and `anchor` into common superclass when
//! target environments support class inheritance
use super::*;

pub type DomSpan = Span;

/// Create an HTML className based on a list of classes. In addition to joining
/// with spaces, we also remove empty classes.
pub fn create_class(classes: &[String]) -> String {
    classes
        .iter()
        .filter(|cls| !cls.is_empty())
        .map(|cls| cls.clone())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Makes an empty DOM node.
fn init_node(
    classes: Option<Vec<String>>,
    options: Option<Options>,
    style: Option<CssStyle>,
) -> HtmlDomNode {
    let classes = classes.unwrap_or_default();
    let style = style.unwrap_or_default();
    let mut node = HtmlDomNode {
        classes,
        attributes: HashMap::new(),
        height: 0,
        depth: 0,
        max_font_size: 0,
        style,
        children: Default::default(),
    };

    if let Some(opts) = options {
        if opts.style.is_tight() {
            node.classes.push("mtight".to_string());
        }
        if let Some(color) = opts.get_color() {
            node.style.insert("color".to_string(), color);
        }
    }

    node
}

/// Convert into an HTML node
fn to_node(node: &HtmlDomNode, tag_name: &str) -> String {
    let mut markup = format!("<{}", tag_name);

    // Add the class
    if !node.classes.is_empty() {
        markup += &format!(" class=\"{}\"", utils::escape(&create_class(&node.classes)));
    }

    let mut styles = String::new();

    // Add the styles, after hyphenation
    for (style, value) in &node.style {
        styles += &format!("{}:{};", utils::hyphenate(style), value);
    }

    if !styles.is_empty() {
        markup += &format!(" style=\"{}\"", utils::escape(&styles));
    }

    // Add the attributes
    for (attr, value) in &node.attributes {
        markup += &format!(" {}=\"{}\"", attr, utils::escape(value));
    }

    markup += ">";

    // Add the markup of the children, also as markup
    for child in &node.children {
        markup += &child.to_markup();
    }

    markup += &format!("</{}>", tag_name);

    markup
}

/// Convert into an HTML markup string
fn to_markup(node: &HtmlDomNode, tag_name: &str) -> String {
    let mut markup = format!("<{}", tag_name);

    // Add the class
    if !node.classes.is_empty() {
        markup += &format!(" class=\"{}\"", utils::escape(&create_class(&node.classes)));
    }

    let mut styles = String::new();

    // Add the styles, after hyphenation
    for (style, value) in &node.style {
        styles += &format!("{}:{};", utils::hyphenate(style), value);
    }

    if !styles.is_empty() {
        markup += &format!(" style=\"{}\"", utils::escape(&styles));
    }

    // Add the attributes
    for (attr, value) in &node.attributes {
        markup += &format!(" {}=\"{}\"", attr, utils::escape(value));
    }

    markup += ">";

    // Add the markup of the children, also as markup
    for child in &node.children {
        markup += &child.to_markup();
    }

    markup += &format!("</{}>", tag_name);

    markup
}

/// Type representing CSS styles
pub type CssStyle = HashMap<String, String>;

/// Represents an HTML DOM node.
#[derive(Debug, Clone)]
pub struct HtmlDomNode {
    pub classes: Vec<String>,
    pub attributes: HashMap<String, String>,
    pub height: usize,
    pub depth: usize,
    pub max_font_size: usize,
    pub style: CssStyle,
    pub children: Vec<VirtualNode>,
}

impl HtmlDomNode {
    /// Checks if the node has a certain class.
    pub fn has_class(&self, class_name: &str) -> bool {
        self.classes.contains(&class_name.to_string())
    }
}

impl PartialEq for HtmlDomNode {
    fn eq(&self, other: &Self) -> bool {
        self.classes == other.classes
            && self.attributes == other.attributes
            && self.height == other.height
            && self.depth == other.depth
            && self.max_font_size == other.max_font_size
            && self.style == other.style
            && self.children == other.children
    }
}

/// Represents a virtual node in the DOM.
#[derive(Debug, Clone, PartialEq)]
pub enum VirtualNode {
    Span(Span),
    Anchor(Anchor),
    Img(Img),
    Symbol(SymbolNode),
    Svg(SvgNode),
    Path(PathNode),
    Line(LineNode),
}

impl VirtualNode {
    pub fn to_node(&self) -> String {
        match self {
            VirtualNode::Span(span) => span.to_node(),
            VirtualNode::Anchor(anchor) => anchor.to_node(),
            VirtualNode::Img(img) => img.to_node(),
            VirtualNode::Symbol(symbol) => symbol.to_node(),
            VirtualNode::Svg(svg) => svg.to_node(),
            VirtualNode::Path(path) => path.to_node(),
            VirtualNode::Line(line) => line.to_node(),
        }
    }

    pub fn to_markup(&self) -> String {
        match self {
            VirtualNode::Span(span) => span.to_markup(),
            VirtualNode::Anchor(anchor) => anchor.to_markup(),
            VirtualNode::Img(img) => img.to_markup(),
            VirtualNode::Symbol(symbol) => symbol.to_markup(),
            VirtualNode::Svg(svg) => svg.to_markup(),
            VirtualNode::Path(path) => path.to_markup(),
            VirtualNode::Line(line) => line.to_markup(),
        }
    }
}

/// Represents a span node containing DOM nodes.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub children: Vec<VirtualNode>,
    pub attributes: HashMap<String, String>,
    pub classes: Vec<String>,
    pub height: usize,
    pub depth: usize,
    pub width: Option<usize>,
    pub max_font_size: usize,
    pub style: CssStyle,
}

/* impl<T: VirtualNode> Span<T> {
    /// Sets an arbitrary attribute on the span.
    pub fn set_attribute(&mut self, attribute: String, value: String) {
        self.attributes.insert(attribute, value);
    }
} */

/// Represents an anchor element with a hyperlink.
#[derive(Debug, Clone, PartialEq)]
pub struct Anchor {
    pub children: Vec<HtmlDomNode>,
    pub attributes: HashMap<String, String>,
    pub classes: Vec<String>,
    pub height: usize,
    pub depth: usize,
    pub max_font_size: usize,
    pub style: CssStyle,
}

impl Anchor {
    /// Sets an attribute for the anchor.
    pub fn set_attribute(&mut self, attribute: String, value: String) {
        self.attributes.insert(attribute, value);
    }
}

/// Represents an image embed element.
#[derive(Debug, Clone, PartialEq)]
pub struct Img {
    pub src: String,
    pub alt: String,
    pub classes: Vec<String>,
    pub height: usize,
    pub depth: usize,
    pub max_font_size: usize,
    pub style: CssStyle,
}

/// Represents a symbol node.
#[derive(Debug, Clone, PartialEq)]
pub struct SymbolNode {
    pub text: String,
    pub height: usize,
    pub depth: usize,
    pub italic: usize,
    pub skew: usize,
    pub width: usize,
    pub max_font_size: usize,
    pub classes: Vec<String>,
    pub style: CssStyle,
}

/// Represents an SVG node.
#[derive(Debug, Clone, PartialEq)]
pub struct SvgNode {
    pub children: Vec<SvgChildNode>,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum SvgChildNode {
    SvgPathNode(PathNode),
    SvgLineNode(LineNode),
}

/// Represents a path node within SVG.
#[derive(Debug, Clone, PartialEq)]
pub struct PathNode {
    pub path_name: String,
    pub alternate: Option<String>,
}

/// Represents a line node within SVG.
#[derive(Debug, Clone, PartialEq)]
pub struct LineNode {
    pub attributes: HashMap<String, String>,
}
