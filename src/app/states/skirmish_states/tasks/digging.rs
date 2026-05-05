use crate::app::widgets::CellWidget;

#[derive(Debug, Clone, PartialEq)]
pub struct DiggingTask<'a> {
    path: Vec<&'a CellWidget>,
}
