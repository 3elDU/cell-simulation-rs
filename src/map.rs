use rand::prelude::*;

use crate::bot::Bot;

/// A structure containing map with all the cells.
/// It is just a wrapper around [`Vec`].
/// But the vec is 1D, and our map is 2D
#[derive(Clone)]
pub struct Map {
    map: Vec<Bot>,
    width: u16,
    height: u16,
}

impl Map {
    pub fn new(width: u16, height: u16) -> Self {
        let mut map = Map {
            map: Vec::with_capacity((width * height).into()),
            width,
            height,
        };

        map.generate_map();

        map
    }

    /// Generates the map, placing a bot in each cell with 20% chance
    pub fn generate_map(&mut self) {
        self.map.clear();
        for y in 0..self.height {
            for x in 0..self.width {
                // 20% chance to generate an alive bot
                let cell_is_alive = thread_rng().gen_bool(1.0 / 5.0);

                let mut bot = Bot::new_empty(x, y);
                if cell_is_alive {
                    bot = Bot::new_random(x, y);
                }

                self.map.push(bot);
            }
        }
    }

    // Returns a cell at the specified coordinates
    pub fn get(&self, x: u16, y: u16) -> Option<&Bot> {
        self.map.get((y * self.width + x) as usize)
    }
    pub fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut Bot> {
        self.map.get_mut((y * self.width + x) as usize)
    }

    /// Set a cell at specified coordinates
    pub fn set(&mut self, x: u16, y: u16, bot: Bot) {
        self.map[(y * self.width + x) as usize] = bot;
    }
}
