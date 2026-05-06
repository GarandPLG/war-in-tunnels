use crate::app::states::skirmish_states::tasks::DiggingTask;

#[derive(Debug, Clone, PartialEq)]
pub enum Tasks {
    Digging(DiggingTask),
}
