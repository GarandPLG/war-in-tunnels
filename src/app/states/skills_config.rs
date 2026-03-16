#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkillsConfigState {
    pub id: usize,
    pub name: &'static str,
    pub selected_skill: usize,
}
