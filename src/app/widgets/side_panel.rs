use crate::app::{
    helpers::block_single_title_helper,
    states::skirmish_states::{
        structures::{Structure, Structures},
        units::{Unit, Units},
    },
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::Color,
    text::Line,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget},
};

#[derive(Debug, Clone, PartialEq)]
pub struct SidePanelWidget<'a> {
    coords: (usize, usize),
    structure: &'a Structures,
    unit: &'a Option<Units>,
    is_unit_marked: bool,
}

impl<'a> SidePanelWidget<'a> {
    pub fn new(
        coords: (usize, usize),
        structure: &'a Structures,
        unit: &'a Option<Units>,
        is_unit_marked: bool,
    ) -> Self {
        Self {
            coords,
            structure,
            unit,
            is_unit_marked,
        }
    }

    fn col_to_letters(&self) -> String {
        let mut col: usize = self.coords.1 + 1;
        let mut letters: Vec<char> = Vec::new();

        while col > 0 {
            letters.push((b'A' + ((col - 1) % 26) as u8) as char);
            col = (col - 1) / 26;
        }

        letters.iter().rev().collect()
    }

    fn get_outer_block(&self) -> Block<'_> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .title(block_single_title_helper(
                format!("{}{}", self.col_to_letters(), self.coords.0),
                Color::Yellow,
            ))
            .title_alignment(Alignment::Center)
    }

    fn get_inner_block<'b>(&self, title: String, color: Color) -> Block<'b> {
        Block::default()
            .borders(Borders::TOP)
            .title(block_single_title_helper(title, color))
            .title_alignment(Alignment::Center)
            .padding(Padding::symmetric(1, 1))
    }

    fn get_area_constraints(&self) -> [Constraint; 2] {
        if self.unit.is_unit() {
            [Constraint::Percentage(50), Constraint::Percentage(50)]
        } else {
            [Constraint::Percentage(100), Constraint::Percentage(0)]
        }
    }
}

impl Widget for SidePanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block: Block<'_> = self.get_outer_block();
        let inner_area: Rect = block.inner(area).inner(Margin {
            horizontal: 2,
            vertical: 1,
        });
        block.render(area, buf);

        let [structure_area, unit_area] =
            Layout::vertical(self.get_area_constraints()).areas(inner_area);

        let mut structure_text: Vec<Line<'_>> = self.structure.text();
        if let Structures::Base(base) = self.structure
            && base.queued_miners() > 0
            && self.unit.is_some()
        {
            structure_text.push(Line::from(base.recruitment_status(true)));
        }
        if let Structures::Mine(mine) = self.structure {
            let staffed: bool = self
                .unit
                .as_ref()
                .is_some_and(|unit: &Units| -> bool { mine.accepts_unit(unit) });
            structure_text.push(Line::from(format!(
                "Metal: {}/s{}",
                mine.production_per_cycle(staffed),
                if staffed { " (Miner)" } else { "" }
            )));
        }
        Paragraph::new(structure_text)
            .alignment(Alignment::Left)
            .block(self.get_inner_block(self.structure.get_name().to_string(), Color::Cyan))
            .render(structure_area, buf);

        if self.unit.is_unit() {
            let mut unit_name: String = self.unit.get_name().to_string();

            if self.is_unit_marked {
                unit_name = format!("> {} <", unit_name);
            }

            Paragraph::new(self.unit.text())
                .alignment(Alignment::Left)
                .block(self.get_inner_block(unit_name, Color::Green))
                .render(unit_area, buf);
        }
    }
}
