pub mod bot;
pub mod color;
pub mod direction;
pub mod gene;
pub mod map;

use bot::Bot;
use map::Map;
use rand::prelude::*;

pub struct Simulation {
    width: usize,
    height: usize,
    iterations: usize,
    map: Map<Bot>,

    selected_bot_coordinates: Option<(usize, usize)>,
    // Keep a copy of the bot even if it no longer exists on the map
    selected_bot: Option<Bot>,
}

impl Simulation {
    /// Create a new simulation with map of given width and height.
    /// Also calls `generate_map()` automatically.
    pub fn new(width: usize, height: usize) -> Self {
        let mut simulation = Simulation {
            width,
            height,
            iterations: 0,
            map: Map::new(width, height),
            selected_bot_coordinates: None,
            selected_bot: None,
        };

        simulation.generate_map();
        simulation
    }

    pub fn generate_map(&mut self) {
        let mut rng = thread_rng();
        for y in 0..self.height {
            for x in 0..self.width {
                // 20% chance to generate an alive bot
                let cell_is_alive = rng.gen_bool(1.0 / 5.0);

                let bot = if cell_is_alive {
                    Bot::new_random(x, y)
                } else {
                    Bot::new_empty(x, y)
                };

                self.map.set(x, y, bot);
            }
        }
    }
    pub fn reset(&mut self) {
        self.iterations = 0;
        self.generate_map();
    }
    pub fn iterations(&self) -> usize {
        self.iterations
    }
    pub fn map(&self) -> &Map<Bot> {
        &self.map
    }

    pub fn select_bot(&mut self, x: usize, y: usize) -> Option<Bot> {
        self.selected_bot_coordinates = Some((x, y));
        let bot = *self.map.get(x, y)?;
        self.selected_bot = Some(bot);
        Some(bot)
    }
    pub fn selected_bot(&self) -> Option<Bot> {
        self.selected_bot
    }

    /// Updates the simulation
    pub fn update(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut bot = *self.map.get(x, y).unwrap();
                let orig_pos = bot.coordinates();

                bot.update(&mut self.map);

                // if bot position was changed, set empty cell at previous position
                if orig_pos != bot.coordinates() {
                    self.map.set(
                        orig_pos.0,
                        orig_pos.1,
                        Bot::new_empty(orig_pos.0, orig_pos.1),
                    );
                }

                // Update coordinates of the selected bot
                if let Some(selected_bot_coordinates) = self.selected_bot_coordinates {
                    if selected_bot_coordinates == orig_pos {
                        self.selected_bot_coordinates = Some(bot.coordinates());
                        self.selected_bot = Some(bot);
                    }
                }

                self.map.set(bot.x(), bot.y(), bot);
            }
        }

        self.iterations += 1;
    }
}
