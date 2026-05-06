use crate::app::states::skirmish_states::units::Units;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct MarkedCells<'a> {
    pub marking_cells: bool,
    pub marked_cells: VecDeque<(usize, usize)>,
    pub selected_unit: &'a Option<Units>,
}

impl MarkedCells<'_> {
    pub fn new() -> Self {
        Self {
            marking_cells: false,
            marked_cells: VecDeque::new(),
            selected_unit: &None,
        }
    }
}
