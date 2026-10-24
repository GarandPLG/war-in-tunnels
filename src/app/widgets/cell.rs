use crate::app::states::skirmish_states::{
    ZoomLevel,
    structures::{Stone, Structure, Structures},
    tasks::Tasks,
    units::{Unit, Units},
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, ToSpan},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

#[derive(Debug, Clone, PartialEq)]
pub struct CellWidget {
    row: usize,
    col: usize,
    selected: bool,
    zoom_level: ZoomLevel,
    structure: Structures,
    marked: bool,
    unit: Option<Units>,
}

impl CellWidget {
    pub fn render_stress(&self, area: Rect, buf: &mut Buffer) -> () {
        if matches!(self.structure, Structures::Stone(_)) {
            self.clone()
                .set_marked(self.selected && self.marked)
                .clone()
                .render(area, buf);
            return;
        }
        let stress: u8 = self.structure.get_stress();
        let red: u8 = (u16::from(stress) * 255 / 100) as u8;
        let color: Color = if self.selected {
            self.fg_color()
        } else {
            Color::Rgb(red, 255 - red, 30)
        };
        let block: Block<'_> = Block::bordered().border_type(BorderType::Rounded).fg(color);
        let inner: Rect = block.inner(area);
        block.render(area, buf);
        if inner.height == 0 || inner.width == 0 {
            return;
        }
        let text: String = if stress == 100 && inner.width < 4 {
            "100".to_string()
        } else {
            format!("{stress}%")
        };
        Paragraph::new(text)
            .fg(color)
            .alignment(Alignment::Center)
            .render(
                Rect::new(inner.x, inner.y + inner.height / 2, inner.width, 1),
                buf,
            );
    }

    pub fn new(
        row: usize,
        col: usize,
        zoom_level: ZoomLevel,
        selected: bool,
        structure: Structures,
        unit: Option<Units>,
    ) -> Self {
        Self {
            row,
            col,
            selected,
            zoom_level,
            marked: false,
            structure,
            unit,
        }
    }

    pub fn set_selected(&mut self, selected: bool) -> &mut Self {
        self.selected = selected;
        self
    }

    pub fn set_zoom_level(&mut self, zoom_level: ZoomLevel) -> &mut Self {
        self.zoom_level = zoom_level;
        self
    }

    pub fn get_structure(&self) -> Structures {
        self.structure
    }

    pub fn get_option_unit(&self) -> Option<Units> {
        self.unit.clone()
    }

    pub fn get_ref_option_unit(&self) -> &Option<Units> {
        &self.unit
    }

    pub fn get_mut_option_unit(&mut self) -> &mut Option<Units> {
        &mut self.unit
    }

    pub fn set_structure(&mut self, structure: Structures) -> &mut Self {
        self.structure = structure;
        self
    }

    pub fn get_marked(&self) -> bool {
        self.marked
    }

    pub fn set_marked(&mut self, marked: bool) -> &mut Self {
        self.marked = marked;
        self
    }

    fn col_to_letters(&self) -> String {
        let mut col: usize = self.col + 1;
        let mut letters: Vec<char> = Vec::new();

        while col > 0 {
            letters.push((b'A' + ((col - 1) % 26) as u8) as char);
            col = (col - 1) / 26;
        }

        letters.iter().rev().collect()
    }

    fn display_coords(&self) -> Span<'_> {
        if self.selected || self.structure != Structures::Stone(Stone::new()) || self.marked {
            format!("{}{}", self.col_to_letters(), self.row).green()
        } else {
            "".to_span()
        }
    }

    fn fg_color(&self) -> Color {
        if self.marked && self.selected {
            Color::Magenta
        } else if self.marked && !self.selected {
            Color::LightMagenta
        } else if !self.marked && self.selected {
            Color::LightYellow
        } else {
            self.structure.get_color()
        }
    }

    fn get_text_area(&self) -> Vec<Line<'_>> {
        let mut lines: Vec<Line<'_>> = self.zoom_level.get_cell_text_area(
            self.structure.get_tag(),
            self.structure.get_level(),
            self.unit
                .get_tag()
                .to_string()
                .fg(self.unit.get_owner().get_color()),
        );
        if matches!(
            self.structure,
            Structures::Stone(_) | Structures::CollapsedTunnel(_)
        ) && self.structure.get_durability() < self.structure.get_max_durability()
        {
            let percent: u32 = u32::from(self.structure.get_durability()) * 100
                / u32::from(self.structure.get_max_durability());
            let middle: usize = lines.len() / 2;
            lines[middle] = Line::from(format!("{percent}%")).cyan();
        }
        if let Structures::MineConstruction(site) = self.structure {
            let middle: usize = lines.len() / 2;
            lines[middle] = Line::from(format!("{}%", site.progress_percent())).cyan();
        }
        if let Some(unit) = &self.unit
            && let Some(Tasks::Moving(task)) = unit.get_ref_tasks().front()
            && let Some(percent) = task.get_movement_progress()
        {
            let middle = lines.len() / 2;
            lines[middle] = Line::from(format!("{percent}%")).fg(unit.get_owner().get_color());
        }
        if let Structures::Base(base) = self.structure
            && base.queued_miners() > 0
        {
            let middle: usize = lines.len() / 2;
            let percent: u16 = base.recruitment_progress();
            let label: String = if self.zoom_level == ZoomLevel::ZoomedOut && percent == 100 {
                percent.to_string()
            } else {
                format!("{percent}%")
            };
            lines[middle] = Line::from(label).cyan();
        }
        if let Structures::Tunnel(tunnel) = self.structure
            && let Some(percent) = tunnel.support_progress()
        {
            let middle: usize = lines.len() / 2;
            lines[middle] = Line::from(format!("{percent}%")).cyan();
        }
        lines
    }

    fn get_block(&self) -> Block<'_> {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(self.fg_color()))
            .title(self.display_coords())
            .border_type(BorderType::Rounded)
    }
}

impl Widget for CellWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.get_text_area())
            .alignment(Alignment::Center)
            .block(self.get_block())
            .render(area, buf);
    }
}
