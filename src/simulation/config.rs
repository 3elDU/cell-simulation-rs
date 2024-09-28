// This is used in array length, so it must be a constant
pub const GENOME_LENGTH: u8 = 32;

#[derive(Clone, Copy, PartialEq)]
pub struct Config {
    // Width and height of the simulation field
    pub width: usize,
    pub height: usize,

    // Cell width and height in pixels
    pub cell_size: usize,

    // % chance that the child will have 1 gene mutated
    pub mutation_percent: f64,

    // Amount of energy the cell spawns with
    pub start_energy: f32,

    // Energy required for cell to reproduce
    pub reproduction_required_energy: f32,

    // Max age the cell can live
    pub cell_max_age: u32,

    // Amount of energy the photosynthesis gives
    pub photosynthesis_energy: f32,

    // Amount energy given from attacking other cell
    pub attack_energy: f32,

    pub movement_cost: f32,

    pub noop_cost: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: 160,
            height: 90,
            cell_size: 8,
            mutation_percent: 25.0,
            start_energy: 5.0,
            reproduction_required_energy: 16.0,
            cell_max_age: 2048,
            photosynthesis_energy: 1.0,
            attack_energy: 5.0,
            movement_cost: 1.0,
            noop_cost: 0.1,
        }
    }
}

impl Config {
    /// Cost of turning left/right
    /// Turn cost is always 1/2 of movement cost
    pub fn turn_cost(&self) -> f32 {
        self.movement_cost / 2.
    }
    /// Cost of "biting" other cell
    /// You need to have 2x movement cost to attack
    pub fn attack_required_energy(&self) -> f32 {
        self.movement_cost * 2.
    }
}
