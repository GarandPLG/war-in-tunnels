pub fn main_menu_option_helper(s: String) -> String {
    if let Some(pos) = s
        .char_indices()
        .filter(|(_, c)| c.is_ascii_uppercase())
        .nth(1)
        .map(|(i, _)| i)
    {
        let (first, second) = s.split_at(pos);
        format!("{first} {second}")
    } else {
        s
    }
}
