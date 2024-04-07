//! This is a module for storing settings passed into KaTeX. It correctly handles
//! default settings.
use super::*;

pub struct Settings {
    /// Render math in display mode, which puts the math in display style
    /// (so \int and \sum are large, for example), and centers the math on
    /// the page on its own line.
    pub display_mode: bool,
    /// Determines the markup language of the output.
    pub output: OutputFormat,
    /// Render display math in leqno style (left-justified tags).
    pub leqno: bool,
    /// Render display math flush left.
    pub fleqn: bool,
    /// Render errors instead of throwing a ParseError exception when
    /// encountering an error.
    pub throw_on_error: bool,
    /// A color string given in the format 'rgb' or 'rrggbb' (no #). This
    /// option determines the color of errors rendered by the -t option.
    pub error_color: String,
    /// Define custom macro of the form '\foo:expansion' (use multiple -m
    /// arguments for multiple macros).
    pub macros: MacroMap,
    /// Specifies a minimum thickness, in ems, for fraction lines, \sqrt
    /// top lines, {array} vertical lines, \hline, \hdashline, \underline,
    /// \overline, and the borders of \fbox, \boxed, and \fcolorbox.
    pub min_rule_thickness: f64,
    /// Makes \color behave like LaTeX's 2-argument \textcolor, instead of
    /// LaTeX's one-argument \color mode change.
    pub color_is_text_color: bool,
    /// Turn on strict / LaTeX faithfulness mode, which throws an error if
    /// the input uses features that are not supported by LaTeX.
    pub strict: bool,
    /// Trust the input, enabling all HTML features such as \url.
    pub trust: bool,
    /// If non-zero, all user-specified sizes, e.g. in \rule{500em}{500em},
    /// will be capped to maxSize ems. Otherwise, elements and spaces can be
    /// arbitrarily large.
    pub max_size: f64,
    /// Limit the number of macro expansions to the specified number, to
    /// prevent e.g. infinite macro loops. If set to Infinity, the macro
    /// expander will try to fully expand as in LaTeX.
    pub max_expand: usize,
    pub global_group: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            display_mode: false,
            output: OutputFormat::Html,
            leqno: false,
            fleqn: false,
            throw_on_error: true,
            error_color: "#cc0000".into(),
            macros: HashMap::default(),
            min_rule_thickness: 0.0,
            color_is_text_color: false,
            strict: false,
            trust: true,
            max_size: f64::INFINITY,
            max_expand: 1000,
            global_group: false,
        }
    }
}

pub type MacroMap = HashMap<String, String>;

pub enum OutputFormat {
    HtmlAndMathml,
    Html,
    Mathml,
}
