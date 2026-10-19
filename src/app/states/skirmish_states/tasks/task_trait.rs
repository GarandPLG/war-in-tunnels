use std::collections::VecDeque;

pub trait Task {
    fn get_icon(&self) -> char;

    fn get_path(&self) -> &VecDeque<(usize, usize)>;

    fn get_path_front_coords(&self) -> &(usize, usize);

    fn get_path_back_coords(&self) -> &(usize, usize);

    fn get_progress(&self) -> f32;

    fn get_total_steps(&self) -> usize;

    fn get_total_progress(&self) -> f32 {
        let total: usize = self.get_total_steps();
        if total == 0 {
            return 100.0;
        }
        let completed: usize = total.saturating_sub(self.get_path().len());
        ((completed as f32 * 100.0 + self.get_progress()) / total as f32).clamp(0.0, 100.0)
    }

    fn progress(&mut self);

    fn is_finished(&self) -> bool;
}
