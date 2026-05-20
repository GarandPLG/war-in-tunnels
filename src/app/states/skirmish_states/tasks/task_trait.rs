use std::collections::VecDeque;

pub trait Task {
    fn get_icon(&self) -> char;

    fn get_path(&self) -> &VecDeque<(usize, usize)>;

    fn get_path_front_coords(&self) -> &(usize, usize);

    fn get_path_back_coords(&self) -> &(usize, usize);

    fn get_progress(&self) -> f32;

    fn progress(&mut self);

    fn is_finished(&self) -> bool;
}
