use super::*;

/// Return whether an element is contained in a list
pub fn contains<T: PartialEq>(list: &[T], elem: &T) -> bool {
    list.contains(elem)
}

/// Provide a default value if a setting is undefined
/// NOTE: Couldn't use `T` as the output type due to facebook/flow#5022.
pub fn deflt<T: Clone>(setting: Option<T>, default_if_undefined: T) -> T {
    setting.unwrap_or(default_if_undefined)
}

/// Hyphenates a string.
pub fn hyphenate(str: &str) -> String {
    let uppercase = regex!(r"([A-Z])");
    uppercase.replace_all(str, "-$1").to_lowercase()
}

/// Escapes text to prevent scripting attacks.
pub fn escape(text: &str) -> String {
    let escape_lookup: HashMap<char, &str> = [
        ('&', "&amp;"),
        ('>', "&gt;"),
        ('<', "&lt;"),
        ('"', "&quot;"),
        ('\'', "&#x27;"),
    ]
    .iter()
    .cloned()
    .collect();

    let escape_regex = regex!(r#"[&><\"']"#);
    escape_regex
        .replace_all(text, |captures: &regex::Captures| {
            escape_lookup
                .get(&captures[0].chars().next().unwrap())
                .unwrap_or(&captures[0])
        })
        .to_string()
}

/// Sometimes we want to pull out the innermost element of a group. In most
/// cases, this will just be the group itself, but when ordgroups and colors have
/// a single element, we want to pull that out.
pub fn get_base_elem(group: &AnyParseNode) -> AnyParseNode {
    if let Some(ord_group) = group.as_ordgroup() {
        if ord_group.body.len() == 1 {
            return get_base_elem(&ord_group.body[0]);
        } else {
            return group.clone();
        }
    } else if let Some(color) = group.as_color() {
        if color.body.len() == 1 {
            return get_base_elem(&color.body[0]);
        } else {
            return group.clone();
        }
    } else if let Some(font) = group.as_font() {
        return get_base_elem(&font.body);
    } else {
        return group.clone();
    }
}

/// TeXbook algorithms often reference "character boxes", which are simply groups
/// with a single character in them. To decide if something is a character box,
/// we find its innermost group, and see if it is a single character.
pub fn is_character_box(group: &AnyParseNode) -> bool {
    let base_elem = get_base_elem(group);

    // These are all they types of groups which hold single characters
    base_elem.is_mathord() || base_elem.is_textord() || base_elem.is_atom()
}

/// Return the protocol of a URL, or "_relative" if the URL does not specify a
/// protocol (and thus is relative), or `None` if URL has invalid protocol
/// (so should be outright rejected).
pub fn protocol_from_url(url: &str) -> Option<String> {
    // Check for possible leading protocol.
    // https://url.spec.whatwg.org/#url-parsing strips leading whitespace
    // (U+20) or C0 control (U+00-U+1F) characters.
    // eslint-disable-next-line no-control-regex
    let protocol = regex!(r"^[\x00-\x20]*([^\\/#?]*?)(:|&#0*58|&#x0*3a|&colon)").captures(url);
    if protocol.is_none() {
        return Some(String::from("_relative"));
    }
    let protocol = protocol.unwrap();
    // Reject weird colons
    if protocol.get(2).unwrap().as_str() != ":" {
        return None;
    }
    // Reject invalid characters in scheme according to
    // https://datatracker.ietf.org/doc/html/rfc3986#section-3.1
    if !regex!(r"^[a-zA-Z][a-zA-Z0-9+\-.]*$").is_match(protocol.get(1).unwrap().as_str()) {
        return None;
    }
    // Lowercase the protocol
    Some(protocol.get(1).unwrap().as_str().to_lowercase())
}
