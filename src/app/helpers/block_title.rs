use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
};

/// Creates a styled title `Line` for a UI block.
///
/// `texts` is a slice of `(String, Color)` tuples. The `String`s are cloned into
/// the spans, so the returned `Line` owns all its data and does not borrow from
/// the caller.
pub fn block_title_helper(
    texts: &[(String, Color)],
    separator: Option<&'static str>,
) -> Line<'static> {
    let mut line: Line<'static> = Line::default();

    line.spans
        .push(Span::from("[ ").fg(Color::Rgb(192, 192, 192)));

    for (i, (text, color)) in texts.iter().enumerate() {
        let span: Span<'_> = Span::styled(text.clone(), Style::new().fg(*color));
        line.spans.push(span);

        if let Some(sep) = separator {
            if i + 1 < texts.len() {
                line.spans
                    .push(Span::from(sep).fg(Color::Rgb(192, 192, 192)));
            }
        }
    }

    line.spans
        .push(Span::from(" ]").fg(Color::Rgb(192, 192, 192)));

    line
}

/// Convenience wrapper for a single‑title line.
///
/// The function builds a slice that contains the supplied `String` and then
/// forwards it to `block_title_helper`. Because `block_title_helper` clones the
/// string, the temporary slice is safe.
pub fn block_single_title_helper(text: String, color: Color) -> Line<'static> {
    block_title_helper(&[(text, color)], None)
}
