//! This file contains information about the options that the Parser carries
//! around with it while parsing. Data is held in an `Options` object, and when
//! recursing, a new `Options` object can be created with the `.with*` and
//! `.reset` functions.

/// Each element contains [textsize, scriptsize, scriptscriptsize].
/// The size mappings are taken from TeX with \normalsize=10pt.
const SIZE_STYLE_MAP: [[f64; 3]; 11] = [
    [5.0, 5.0, 5.0],       // size1: [5, 5, 5]              \tiny
    [6.0, 5.0, 5.0],       // size2: [6, 5, 5]
    [7.0, 5.0, 5.0],       // size3: [7, 5, 5]              \scriptsize
    [8.0, 6.0, 5.0],       // size4: [8, 6, 5]              \footnotesize
    [9.0, 6.0, 5.0],       // size5: [9, 6, 5]              \small
    [10.0, 7.0, 5.0],      // size6: [10, 7, 5]             \normalsize
    [12.0, 8.0, 6.0],      // size7: [12, 8, 6]             \large
    [14.4, 10.0, 7.0],     // size8: [14.4, 10, 7]          \Large
    [17.28, 12.0, 10.0],   // size9: [17.28, 12, 10]        \LARGE
    [20.74, 14.4, 12.0],   // size10: [20.74, 14.4, 12]     \huge
    [24.88, 20.74, 17.28], // size11: [24.88, 20.74, 17.28] \HUGE
];

/// fontMetrics.js:getGlobalMetrics also uses size indexes, so if
/// you change size indexes, change that function.
const SIZE_MULTIPLIERS: [f64; 11] = [0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.2, 1.44, 1.728, 2.074, 2.488];

#[derive(Clone)]
pub enum FontWeight {
    TextBF,
    TextMD,
    None,
}

#[derive(Clone)]
pub enum FontShape {
    TextIt,
    TextUp,
}

/// This is the main options class. It contains the current style, size, color,
/// and font.
///
/// Options objects should not be modified. To create a new Options with
/// different properties, call a `.having*` method.
pub struct Options {
    // pub style: StyleInterface,
    pub color: Option<String>,
    pub size: usize,
    pub text_size: usize,
    pub phantom: bool,
    /// A font family applies to a group of fonts (i.e. SansSerif), while a font
    /// represents a specific font (i.e. SansSerif Bold).
    /// See: https://tex.stackexchange.com/questions/22350/difference-between-textrm-and-mathrm
    pub font: String,
    pub font_family: String,
    pub font_weight: FontWeight,
    pub font_shape: FontShape,
    pub size_multiplier: f64,
    pub max_size: usize,
    pub min_rule_thickness: usize,
    // pub font_metrics: Option<FontMetrics>,
}

impl Options {
    /// The base size index.
    const BASESIZE: usize = 6;

    pub fn new(data: OptionsData) -> Self {
        let size_multiplier = SIZE_MULTIPLIERS[data.size - 1];
        Options {
            // style: data.style,
            color: data.color,
            size: data.size,
            text_size: data.text_size,
            phantom: data.phantom,
            font: data.font.unwrap_or_default(),
            font_family: data.font_family.unwrap_or_default(),
            font_weight: data.font_weight.unwrap_or(FontWeight::None),
            font_shape: data.font_shape.unwrap_or(FontShape::TextUp),
            size_multiplier,
            max_size: data.max_size,
            min_rule_thickness: data.min_rule_thickness,
            // font_metrics: None,
        }
    }

    /// Returns a new options object with the same properties as "this".  Properties
    /// from "extension" will be copied to the new options object.
    pub fn extend(&self, extension: OptionsData) -> Self {
        let mut data = extension;
        data.style = self.style;
        data.size = self.size;
        data.text_size = self.text_size;
        data.color = self.color.clone();
        data.phantom = self.phantom;
        data.font = Some(self.font.clone());
        data.font_family = Some(self.font_family.clone());
        data.font_weight = Some(self.font_weight.clone());
        data.font_shape = Some(self.font_shape.clone());
        Options::new(data)
    }

    // Implement other methods similarly
}

pub struct OptionsData {
    // pub style: StyleInterface,
    pub color: Option<String>,
    pub size: usize,
    pub text_size: usize,
    pub phantom: bool,
    pub font: Option<String>,
    pub font_family: Option<String>,
    pub font_weight: Option<FontWeight>,
    pub font_shape: Option<FontShape>,
    pub max_size: usize,
    pub min_rule_thickness: usize,
}

impl Default for OptionsData {
    fn default() -> Self {
        OptionsData {
            // style: Default::default(),
            color: None,
            size: Options::BASESIZE,
            text_size: Options::BASESIZE,
            phantom: false,
            font: None,
            font_family: None,
            font_weight: None,
            font_shape: None,
            max_size: 0,
            min_rule_thickness: 0,
        }
    }
}
