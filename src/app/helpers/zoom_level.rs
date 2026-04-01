use crate::app::states::ZoomLevel;

pub enum CellSizes {
    Width,
    Height,
}

pub fn cell_size_helper(cell_size: CellSizes, zoom_level: ZoomLevel) -> usize {
    match (cell_size, zoom_level) {
        (CellSizes::Width, ZoomLevel::ZoomedIn) => 13,
        (CellSizes::Width, ZoomLevel::Default) => 9,
        (CellSizes::Width, ZoomLevel::ZoomedOut) => 5,
        (CellSizes::Height, ZoomLevel::ZoomedIn) => 7,
        (CellSizes::Height, ZoomLevel::Default) => 5,
        (CellSizes::Height, ZoomLevel::ZoomedOut) => 3,
    }
}
