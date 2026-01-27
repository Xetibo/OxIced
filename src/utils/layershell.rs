use iced_layershell::reexport::Anchor;

fn anchor_from_string(anchor_str: &str) -> Anchor {
    match anchor_str.to_lowercase().as_str() {
        "top" => Anchor::Top,
        "bottom" => Anchor::Bottom,
        "right" => Anchor::Right,
        "left" => Anchor::Left,
        _ => Anchor::Top,
    }
}

pub fn anchor_from_strings(anchor_strs: Vec<&str>) -> Anchor {
    let mut anchor = Anchor::empty();
    for anchor_str in anchor_strs {
        anchor |= anchor_from_string(anchor_str);
    }
    anchor
}
