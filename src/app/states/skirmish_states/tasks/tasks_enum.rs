use crate::app::states::skirmish_states::tasks::DiggingTask;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tasks {
    Digging(DiggingTask),
}
