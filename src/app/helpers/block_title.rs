use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
};

/// Creates a styled title `Line` for a UI block.
///
/// This helper builds a `Line` that looks like:
/// ```text
/// [ <title‑1> <separator> <title‑2> ... ]
/// ```
/// where each title fragment is rendered in the color supplied by the caller,
/// and the surrounding brackets and any separator are rendered in gray.
///
/// # Arguments
///
/// * `texts` – A slice of tuples. Each tuple contains a title fragment (`&str`)
///   and a `Color` that should be applied to that fragment. The order of the
///   tuples determines the order of the fragments in the final line.
///
/// * `separator` – An optional string that will be inserted (in gray) between
///   successive title fragments. If `None` the fragments are concatenated
///   directly without any separator.
///
/// # Returns
///
/// A `Line<'a>` that starts with a gray “\[ ”, contains the colored title
/// fragments separated (if requested) by a gray separator, and ends with a
/// gray “ \]”. The line can be passed directly to widgets such as `Block::title`.
///
/// # Example
///
/// ```rust
/// use war_in_tunnels::app::helpers::block_title::block_title_helper;
/// use ratatui::style::Color;
/// use ratatui::text::Line;
///
/// let title: Line<'_> = block_title_helper(
///     &[
///         ("Skirmish", Color::Magenta),
///         ("Map", Color::Green),
///     ],
///     Some(" - ")
/// );
/// // `title` now represents: [ Skirmish - Map ] with the appropriate colors.
/// ```
pub fn block_title_helper<'a>(texts: &[(&'a str, Color)], separator: Option<&'a str>) -> Line<'a> {
    let mut line: Line<'a> = Line::default();

    line.spans.push(Span::from("[ ").gray());

    for (i, (text, color)) in texts.iter().enumerate() {
        let span: Span<'a> = Span::styled(*text, Style::new().fg(*color));

        line.spans.push(span);

        if let Some(sep) = separator {
            if i + 1 < texts.len() {
                line.spans.push(Span::from(sep).gray());
            }
        }
    }

    line.spans.push(Span::from(" ]").gray());

    line
}

/// Convenience wrapper for `block_title_helper` that creates a title line for a
/// single piece of text.
///
/// This function builds a `Line` that looks like:
/// ```text
/// [ <text> ]
/// ```
/// where `<text>` is rendered with the supplied color, and the surrounding
/// brackets are rendered in gray. It simply forwards the arguments to
/// `block_title_helper` with `separator` set to `None`.
///
/// # Arguments
///
/// * `text` – The title string to display.
///
/// * `color` – The `Color` that should be applied to the title text.
///
/// # Returns
///
/// A `Line<'a>` containing the grey brackets and the coloured title text,
/// ready to be passed to widgets such as `Block::title`.
///
/// # Example
///
/// ```rust
/// use war_in_tunnels::app::helpers::block_title::block_single_title_helper;
/// use ratatui::style::Color;
/// use ratatui::text::Line;
///
/// let title: Line<'_> = block_single_title_helper("Keybindings", Color::Magenta);
/// // `title` now represents: [ Keybindings ] with “Keybindings” coloured magenta.
/// ```
pub fn block_single_title_helper<'a>(text: &'a str, color: Color) -> Line<'a> {
    block_title_helper(&[(text, color)], None)
}
