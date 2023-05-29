use super::bot;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

pub struct Simulation {
    width: usize,
    height: usize,
    map: HashMap<Coordinates, bot::Bot>,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        let mut simulation = Simulation {
            width,
            height,
            map: HashMap::new(),
        };
        simulation.generate_map();

        simulation
    }

    // Generates the map, placing a bot in each cell with 20% chance
    pub fn generate_map(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                // 20% chance to generate an alive bot
                let cell_is_alive = thread_rng().gen_bool(1.0 / 5.0);

                let mut bot = bot::Bot::new_empty(x, y);
                if cell_is_alive {
                    bot = bot::Bot::new_random(x, y);
                }

                self.map.insert(Coordinates { x, y }, bot);
            }
        }
    }

    // Returns a reference to cell at given coordinates
    pub fn cell_at(&self, x: usize, y: usize) -> Option<&bot::Bot> {
        self.map.get(&Coordinates { x, y })
    }
    // Returns a mutable reference to cell at given coordinates
    pub fn cell_at_mut(&mut self, x: usize, y: usize) -> Option<&mut bot::Bot> {
        self.map.get_mut(&Coordinates { x, y })
    }

    // Sets a cell at given coordinates
    pub fn set_cell_at(&mut self, x: usize, y: usize, cell: bot::Bot) {
        if x > self.width || y > self.height {
            return;
        }
        self.map.insert(Coordinates { x, y }, cell);
    }

    // Updates the simulation
    pub fn update(&mut self) {
        let mut bots_to_update = Vec::new();

        // Iterate over all the bots, and check which of them should be updated
        self.map.iter().for_each(|(coordinates, bot)| {
            if bot.should_update() {
                bots_to_update.push(*coordinates);
            }
        });

        for coordinates in bots_to_update {
            let mut bot = *self.map.get(&coordinates).unwrap();
            let orig_pos = (bot.x, bot.y);

            bot.update(self);

            // if bot position has changed, replace previous position with empty cell
            if orig_pos != (bot.x, bot.y) {
                self.set_cell_at(
                    orig_pos.0,
                    orig_pos.1,
                    bot::Bot::new_empty(orig_pos.0, orig_pos.1),
                );
            }

            self.set_cell_at(bot.x, bot.y, bot);
        }
    }
}
