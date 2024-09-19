// Width and height of the simulation field
pub const SIMULATION_WIDTH: usize = 160;
pub const SIMULATION_HEIGHT: usize = 90;

// Cell width and height in pixels
pub const CELL_SIZE: usize = 8;

// % chance that the child will have 1 gene mutated
pub const MUTATION_PERCENT: f64 = 5.0;

pub const GENOME_LENGTH: u8 = 32;

// Amount of energy the cell spawns with
pub const START_ENERGY: f32 = 5.0;

// Energy required for cell to reproduce
pub const REPRODUCTION_REQUIRED_ENERGY: f32 = 16.0;

// Max age the cell can live
pub const CELL_MAX_AGE: u32 = 2048;

// Amount of energy the photosynthesis gives
pub const PHOTOSYNTHESIS_ENERGY: f32 = 1.25;

// Amount energy given from attacking other cell
pub const ATTACK_ENERGY: f32 = PHOTOSYNTHESIS_ENERGY * 4.0;

pub const MOVEMENT_COST: f32 = 1.0;

// Cost of turning left/right
pub const TURN_COST: f32 = MOVEMENT_COST * 0.5;

// Cost of "biting" other cell
pub const ATTACK_REQUIRED_ENERGY: f32 = MOVEMENT_COST * 2.0;

pub const NOOP_COST: f32 = 0.1;
