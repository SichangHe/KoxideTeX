use super::*;

/// Represents an alignment specification.
#[derive(Debug, Clone, PartialEq)]
pub struct AlignSpec {
    // Define your AlignSpec struct fields here
}

/// Represents a column separation type.
#[derive(Debug, Clone, PartialEq)]
pub enum ColSeparationType {
    // Define your ColSeparationType variants here
}

/// Represents a measurement.
#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    // Define your Measurement struct fields here
}

/// Represents an atom.
#[derive(Debug, Clone, PartialEq)]
pub struct Atom {
    // Define your Atom struct fields here
}

/// Represents a parse node.
#[derive(Debug, Clone, PartialEq)]
pub enum AnyParseNode {
    Array(ArrayParseNode),
    CdLabel(CdLabelParseNode),
    CdLabelParent(CdLabelParentParseNode),
    Color(ColorParseNode),
    ColorToken(ColorTokenParseNode),
    Op(OpParseNode),
    OrdGroup(OrdGroupParseNode),
    Raw(RawParseNode),
    Size(SizeParseNode),
    Styling(StylingParseNode),
    SupSub(SupSubParseNode),
    Tag(TagParseNode),
    Text(TextParseNode),
    Url(UrlParseNode),
    Verb(VerbParseNode),
    Accent(AccentParseNode),
    AccentUnder(AccentUnderParseNode),
    Cr(CrParseNode),
    DelimSizing(DelimSizingParseNode),
    Enclose(EncloseParseNode),
    Environment(EnvironmentParseNode),
    Font(FontParseNode),
    Genfrac(GenfracParseNode),
    Hbox(HboxParseNode),
    HorizBrace(HorizBraceParseNode),
    Href(HrefParseNode),
    Html(HtmlParseNode),
    HtmlMathml(HtmlMathmlParseNode),
    Includegraphics(IncludegraphicsParseNode),
    Infix(InfixParseNode),
    Internal(InternalParseNode),
    Kern(KernParseNode),
    Lap(LapParseNode),
    LeftRight(LeftRightParseNode),
    LeftRightRight(LeftRightRightParseNode),
    MathChoice(MathChoiceParseNode),
    Middle(MiddleParseNode),
    Mclass(MclassParseNode),
    Operatorname(OperatornameParseNode),
    Overline(OverlineParseNode),
    Phantom(PhantomParseNode),
    Hphantom(HphantomParseNode),
    Vphantom(VphantomParseNode),
    Pmb(PmbParseNode),
    Raisebox(RaiseboxParseNode),
    Rule(RuleParseNode),
    Sizing(SizingParseNode),
    Smash(SmashParseNode),
    Sqrt(SqrtParseNode),
    Underline(UnderlineParseNode),
    Vcenter(VcenterParseNode),
    XArrow(XArrowParseNode),
}

/// Represents an array parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub col_separation_type: Option<ColSeparationType>,
    pub hskip_before_and_after: Option<bool>,
    pub add_jot: Option<bool>,
    pub cols: Option<Vec<AlignSpec>>,
    pub arraystretch: f64,
    pub body: Vec<Vec<AnyParseNode>>,
    pub row_gaps: Vec<Option<Measurement>>,
    pub h_lines_before_row: Vec<Vec<bool>>,
    pub tags: Option<Vec<bool>>,
    pub leqno: Option<bool>,
    pub is_cd: Option<bool>, // Define your ArrayParseNode struct fields here
}

/// Represents a cd label parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct CdLabelParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub side: String,
    pub label: Box<AnyParseNode>, // Define your CdLabelParseNode struct fields here
}

/// Represents a cd label parent parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct CdLabelParentParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub fragment: Box<AnyParseNode>, // Define your CdLabelParentParseNode struct fields here
}

/// Represents a color parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct ColorParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub color: String,
    pub body: Vec<AnyParseNode>, // Define your ColorParseNode struct fields here
}

/// Represents a color token parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct ColorTokenParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub color: String, // Define your ColorTokenParseNode struct fields here
}

/// Represents an op parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct OpParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub limits: bool,
    pub always_handle_sup_sub: Option<bool>,
    pub suppress_base_shift: Option<bool>,
    pub parent_is_sup_sub: bool,
    pub symbol: bool,
    pub name: Option<String>,
    pub body: Option<Vec<AnyParseNode>>, // Define your OpParseNode struct fields here
}

/// Represents an ord group parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct OrdGroupParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub body: Vec<AnyParseNode>,
    pub semisimple: Option<bool>, // Define your OrdGroupParseNode struct fields here
}

/// Represents a raw parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct RawParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub string: String, // Define your RawParseNode struct fields here
}

/// Represents a size parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct SizeParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub value: Measurement,
    pub is_blank: bool, // Define your SizeParseNode struct fields here
}

/// Represents a styling parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct StylingParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub style: String,
    pub body: Vec<AnyParseNode>, // Define your StylingParseNode struct fields here
}

/// Represents a supsub parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct SupSubParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub base: Option<Box<AnyParseNode>>,
    pub sup: Option<Box<AnyParseNode>>,
    pub sub: Option<Box<AnyParseNode>>, // Define your SupSubParseNode struct fields here
}

/// Represents a tag parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct TagParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub body: Vec<AnyParseNode>,
    pub tag: Vec<AnyParseNode>, // Define your TagParseNode struct fields here
}

/// Represents a text parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct TextParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub body: Vec<AnyParseNode>,
    pub font: Option<String>, // Define your TextParseNode struct fields here
}

/// Represents a URL parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct UrlParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub url: String, // Define your UrlParseNode struct fields here
}

/// Represents a verb parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct VerbParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub body: String,
    pub star: bool, // Define your VerbParseNode struct fields here
}

/// Represents an accent parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct AccentParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub label: String,
    pub is_stretchy: Option<bool>,
    pub is_shifty: Option<bool>,
    pub base: Box<AnyParseNode>, // Define your AccentParseNode struct fields here
}

/// Represents an under-accent parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct AccentUnderParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub label: String,
    pub is_stretchy: Option<bool>,
    pub is_shifty: Option<bool>,
    pub base: Box<AnyParseNode>, // Define your AccentUnderParseNode struct fields here
}

/// Represents a carriage return parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct CrParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub new_line: bool,
    pub size: Option<Measurement>, // Define your CrParseNode struct fields here
}

/// Represents a delimiter sizing parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct DelimSizingParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub size: i64,
    pub mclass: String,
    pub delim: String, // Define your DelimSizingParseNode struct fields here
}

/// Represents an enclose parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct EncloseParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub label: String,
    pub background_color: Option<String>,
    pub border_color: Option<String>,
    pub body: Box<AnyParseNode>, // Define your EncloseParseNode struct fields here
}

/// Represents an environment parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct EnvironmentParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub name: String,
    pub name_group: Box<AnyParseNode>, // Define your EnvironmentParseNode struct fields here
}

/// Represents a font parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct FontParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub font: String,
    pub body: Box<AnyParseNode>, // Define your FontParseNode struct fields here
}

/// Represents a general fraction parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct GenfracParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub continued: bool,
    pub numer: Box<AnyParseNode>,
    pub denom: Box<AnyParseNode>,
    pub has_bar_line: bool,
    pub left_delim: Option<String>,
    pub right_delim: Option<String>,
    pub size: String,
    pub bar_size: Option<Measurement>, // Define your GenfracParseNode struct fields here
}

/// Represents an hbox parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct HboxParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub body: Vec<AnyParseNode>, // Define your HboxParseNode struct fields here
}

/// Represents a horizontal brace parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct HorizBraceParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub label: String,
    pub is_over: bool,
    pub base: Box<AnyParseNode>, // Define your HorizBraceParseNode struct fields here
}

/// Represents an href parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct HrefParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub href: String,
    pub body: Vec<AnyParseNode>, // Define your HrefParseNode struct fields here
}

/// Represents an html parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct HtmlParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub attributes: HashMap<String, String>,
    pub body: Vec<AnyParseNode>, // Define your HtmlParseNode struct fields here
}

/// Represents an HTML-MathML parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct HtmlMathmlParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub html: Vec<AnyParseNode>,
    pub mathml: Vec<AnyParseNode>, // Define your HtmlMathmlParseNode struct fields here
}

/// Represents an includegraphics parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct IncludegraphicsParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub alt: String,
    pub width: Measurement,
    pub height: Measurement,
    pub totalheight: Measurement,
    pub src: String, // Define your IncludegraphicsParseNode struct fields here
}

/// Represents an infix parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct InfixParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub replace_with: String,
    pub size: Option<Measurement>,
    pub token: Option<Token>, // Define your InfixParseNode struct fields here
}

/// Represents an internal parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct InternalParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>, // Define your InternalParseNode struct fields here
}

/// Represents a kern parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct KernParseNode {
    pub type_: String,

    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub dimension: Measurement, // Define your KernParseNode struct fields here
}

/// Represents a lap parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct LapParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub alignment: String,
    pub body: Box<AnyParseNode>, // Define your LapParseNode struct fields here
}

/// Represents a left-right parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct LeftRightParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub body: Vec<AnyParseNode>,
    pub left: String,
    pub right: String,
    pub right_color: Option<String>, // Define your LeftRightParseNode struct fields here
}

/// Represents a left-right-right parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct LeftRightRightParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub delim: String,
    pub color: Option<String>, // Define your LeftRightRightParseNode struct fields here
}

/// Represents a math choice parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct MathChoiceParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub display: Vec<AnyParseNode>,
    pub text: Vec<AnyParseNode>,
    pub script: Vec<AnyParseNode>,
    pub scriptscript: Vec<AnyParseNode>, // Define your MathChoiceParseNode struct fields here
}

/// Represents a middle parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct MiddleParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub delim: String, // Define your MiddleParseNode struct fields here
}

/// Represents a math class parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct MclassParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub mclass: String,
    pub body: Vec<AnyParseNode>,
    pub is_character_box: bool, // Define your MclassParseNode struct fields here
}

/// Represents an operator name parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct OperatornameParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub body: Vec<AnyParseNode>,
    pub always_handle_sup_sub: bool,
    pub limits: bool,
    pub parent_is_sup_sub: bool, // Define your OperatornameParseNode struct fields here
}

/// Represents an overline parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct OverlineParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub body: Box<AnyParseNode>, // Define your OverlineParseNode struct fields here
}

/// Represents a phantom parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct PhantomParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub body: Vec<AnyParseNode>, // Define your PhantomParseNode struct fields here
}

/// Represents a horizontal phantom parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct HphantomParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub body: Box<AnyParseNode>, // Define your HphantomParseNode struct fields here
}

/// Represents a vertical phantom parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct VphantomParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub body: Box<AnyParseNode>, // Define your VphantomParseNode struct fields here
}

/// Represents a padded mbox parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct PmbParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub mclass: String,
    pub body: Vec<AnyParseNode>, // Define your PmbParseNode struct fields here
}

/// Represents a raised box parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct RaiseboxParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub dy: Measurement,
    pub body: Box<AnyParseNode>, // Define your RaiseboxParseNode struct fields here
}

/// Represents a rule parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct RuleParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub shift: Option<Measurement>,
    pub width: Measurement,
    pub height: Measurement, // Define your RuleParseNode struct fields here
}

/// Represents a sizing parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct SizingParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub size: f64,
    pub body: Vec<AnyParseNode>, // Define your SizingParseNode struct fields here
}

/// Represents a smash parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct SmashParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub body: Box<AnyParseNode>,
    pub smash_height: bool,
    pub smash_depth: bool, // Define your SmashParseNode struct fields here
}

/// Represents a square root parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct SqrtParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub body: Box<AnyParseNode>,
    pub index: Option<Box<AnyParseNode>>, // Define your SqrtParseNode struct fields here
}

/// Represents an underline parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct UnderlineParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub body: Box<AnyParseNode>, // Define your UnderlineParseNode struct fields here
}

/// Represents a vertical centering parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct VcenterParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub body: Box<AnyParseNode>, // Define your VcenterParseNode struct fields here
}

/// Represents an x-arrow parse node.
#[derive(Debug, Clone, PartialEq)]
pub struct XArrowParseNode {
    pub type_: String,
    pub mode: String,
    pub loc: Option<SourceLocation>,
    pub label: String,
    pub body: Box<AnyParseNode>,
    pub below: Option<Box<AnyParseNode>>, // Define your XArrowParseNode struct fields here
}
