#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MainMenuState {
  pub id: usize,
  pub name: &'static str,
  pub selected_view: usize,
}
