use super::*;

fn options_from_settings(settings: &Settings) -> Options {
    Options {
        style: if settings.display_mode {
            Style::DISPLAY
        } else {
            Style::TEXT
        },
        max_size: settings.max_size,
        min_rule_thickness: settings.min_rule_thickness,
    }
}

fn display_wrap(node: DomSpan, settings: &Settings) -> DomSpan {
    if settings.display_mode {
        let mut classes = vec!["katex-display"];
        if settings.leqno {
            classes.push("leqno");
        }
        if settings.fleqn {
            classes.push("fleqn");
        }
        let node = make_span(&classes, vec![node]);
        return node;
    }
    node
}

pub fn build_tree(tree: Vec<AnyParseNode>, expression: &str, settings: &Settings) -> DomSpan {
    let options = options_from_settings(settings);
    let katex_node: DomSpan;

    if settings.output == "mathml" {
        return build_math_ml(&tree, expression, &options, settings.display_mode, true);
    } else if settings.output == "html" {
        let html_node = build_html(&tree, &options);
        katex_node = make_span(&["katex"], vec![html_node]);
    } else {
        let mathml_node = build_math_ml(&tree, expression, &options, settings.display_mode, false);
        let html_node = build_html(&tree, &options);
        katex_node = make_span(&["katex"], vec![mathml_node, html_node]);
    }

    display_wrap(katex_node, settings)
}

pub fn build_html_tree(tree: Vec<AnyParseNode>, expression: &str, settings: &Settings) -> DomSpan {
    let options = options_from_settings(settings);
    let html_node = build_html(&tree, &options);
    let katex_node = make_span(&["katex"], vec![html_node]);
    display_wrap(katex_node, settings)
}
