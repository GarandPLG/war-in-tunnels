use ratatui::style::Color;

pub trait Structure {
    fn get_tag(&self) -> char;
    fn get_color(&self) -> Color;
    fn get_level(&self) -> char;
    fn get_stress(&self) -> u8;
    fn get_durability(&self) -> u16;
    fn get_name(&self) -> &'static str;
}
