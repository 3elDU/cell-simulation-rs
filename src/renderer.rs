use crate::{
    simulation::{bot::Bot, color::Color},
    Config,
};

#[derive(PartialEq)]
pub enum RenderingMode {
    /// Show original cell colors,
    Normal,
    /// More energy the cell has, brightner yellow color
    Energy,
    /// Older cells have darker color
    Lifetime,
}

impl RenderingMode {
    pub fn render(&self, bot: &Bot, config: &Config) -> Color {
        let reproduction_required_energy = config.reproduction_required_energy as f32;

        match self {
            Self::Normal => bot.color,
            Self::Energy => {
                if bot.energy < reproduction_required_energy * 5. {
                    Color::new(255, 255, 0)
                        * (bot.energy as f64 / config.reproduction_required_energy as f64 * 5.)
                } else {
                    Color::new(0, 255, 0)
                        * ((bot.energy as f64 - reproduction_required_energy as f64 * 5.) / 255.)
                }
            }
            Self::Lifetime => Color::new(
                10 + ((bot.age as f64 / config.cell_max_age as f64) * 245.) as u8,
                0,
                0,
            ),
        }
    }
}
