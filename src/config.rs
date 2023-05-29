// Width and height of the simulation field
pub const SIMULATION_WIDTH: usize = 96;
pub const SIMULATION_HEIGHT: usize = 96;

// Cell width and height in pixels
pub const CELL_SIZE: usize = 10;

// % chance that the child will have 1 gene mutated
pub const MUTATION_PERCENT: f64 = 50.0;

pub const GENOME_LENGTH: usize = 32;

// Amount of energy the cell spawns with
pub const START_ENERGY: f64 = 5.0;

// Energy required for cell to reproduce
pub const REPRODUCTION_REQUIRED_ENERGY: f64 = 16.0;

// Max age the cell can live
pub const CELL_MAX_AGE: usize = 2048;

// Amount of energy the photosynthesis gives
pub const PHOTOSYNTHESIS_ENERGY: f64 = 1.0;

// Fraction of energy given from attacking other cell
pub const ATTACK_ENERGY: f64 = 0.5;

pub const MOVEMENT_COST: f64 = 1.0;

// Cost of turning left/right
pub const TURN_COST: f64 = MOVEMENT_COST * 0.5;

// Cost of "biting" other cell
pub const ATTACK_REQUIRED_ENERGY: f64 = MOVEMENT_COST * 2.0;

pub const NOOP_COST: f64 = 0.1;
