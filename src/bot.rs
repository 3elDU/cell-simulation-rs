use num_derive::FromPrimitive;
use num_derive::ToPrimitive;
use num_traits::FromPrimitive;
use rand::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::color::Color;
use crate::config;
use crate::gene;
use crate::map::Map;

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive, Serialize, Deserialize)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    // Generates a random direction
    fn new_random() -> Self {
        FromPrimitive::from_u8(thread_rng().gen_range(0..(Direction::Down as u8))).unwrap()
    }

    // Applies directional movement to given coordinates
    fn apply_direction(&self, x: u16, y: u16) -> (u16, u16) {
        match self {
            Self::Left => {
                if x == 0 {
                    (config::SIMULATION_WIDTH - 1, y)
                } else {
                    (x - 1, y)
                }
            }
            Self::Right => {
                if x == config::SIMULATION_WIDTH - 1 {
                    (0, y)
                } else {
                    (x + 1, y)
                }
            }
            Self::Up => {
                if y == 0 {
                    (x, config::SIMULATION_HEIGHT - 1)
                } else {
                    (x, y - 1)
                }
            }
            Self::Down => {
                if y == config::SIMULATION_HEIGHT - 1 {
                    (x, 0)
                } else {
                    (x, y + 1)
                }
            }
        }
    }

    // 'Rotates' direction to the left, returning a new one
    fn left(&self) -> Self {
        match self {
            Self::Down => Direction::Right,
            Self::Right => Direction::Up,
            Self::Up => Direction::Left,
            Self::Left => Direction::Down,
        }
    }

    // 'Rotates' direction to the right, returning a new one
    fn right(&self) -> Self {
        match self {
            Self::Left => Direction::Up,
            Self::Up => Direction::Right,
            Self::Right => Direction::Down,
            Self::Down => Direction::Left,
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Bot {
    pub alive: bool,
    pub empty: bool,

    pub x: u16,
    pub y: u16,
    pub energy: f32,
    pub direction: Direction,
    pub color: Color,
    pub age: u32,

    pub genome: [gene::Gene; config::GENOME_LENGTH as usize],
    current_instruction: u8,
}

impl std::fmt::Debug for Bot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bot")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("alive", &self.alive)
            .field("empty", &self.empty)
            .field("energy", &self.energy)
            .field("direction", &self.direction)
            .finish()
    }
}

impl Bot {
    // Generates an alive bot with random color and genome
    pub fn new_random(x: u16, y: u16) -> Self {
        let mut genome = [gene::Gene::default(); config::GENOME_LENGTH as usize];

        // Generate the genome
        for gene in genome.iter_mut() {
            *gene = gene::Gene::new_random();
        }

        Bot {
            alive: true,
            empty: false,

            x,
            y,
            energy: config::START_ENERGY,
            direction: Direction::new_random(),
            age: 0,

            color: Color::new(rand::random(), rand::random(), rand::random()),
            genome,
            current_instruction: 0,
        }
    }

    // Generates an empty bot
    pub fn new_empty(x: u16, y: u16) -> Self {
        Bot {
            alive: false,
            empty: true,

            x,
            y,
            energy: 0.0,
            direction: Direction::Left,
            age: 0,

            color: Color::BLACK,
            genome: [gene::Gene::default(); 32],
            current_instruction: 0,
        }
    }

    pub fn x(&self) -> u16 {
        self.x
    }
    pub fn y(&self) -> u16 {
        self.y
    }
    pub fn coordinates(&self) -> (u16, u16) {
        (self.x, self.y)
    }
    pub fn set_coordinates(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    // Whether a bot should update
    pub fn should_update(&self) -> bool {
        self.alive
    }

    // Returns reference to the current instruction
    pub fn current_instruction(&self) -> &gene::Gene {
        &self.genome[self.current_instruction as usize]
    }

    // Whether a bot is a dead cell
    pub fn is_dead(&self) -> bool {
        !self.alive && !self.empty
    }

    // Update a bot
    // Bot needs a mutable reference to the map to be able to look up other bots and change their fields
    // Example: Attacking other bots (changing their energy), or schecking the bot in front
    pub fn update(&mut self, map: &mut Map) {
        if !self.alive {
            return;
        }

        let mut next_instruction = self.current_instruction + 1;
        let (looking_x, looking_y) = self.direction.apply_direction(self.x, self.y);

        let cell_in_front = map.get_mut(looking_x, looking_y).unwrap();

        use gene::Instruction;
        match self.current_instruction().instruction {
            Instruction::TurnLeft => {
                self.direction = self.direction.left();
                self.energy -= config::TURN_COST;
            }
            Instruction::TurnRight => {
                self.direction = self.direction.right();
                self.energy -= config::TURN_COST;
            }
            Instruction::MoveForwards => {
                if cell_in_front.empty {
                    self.x = looking_x;
                    self.y = looking_y;
                    self.energy -= config::MOVEMENT_COST;
                }
            }

            Instruction::Photosynthesis => {
                self.energy += config::PHOTOSYNTHESIS_ENERGY;
            }
            Instruction::GiveEnergy => {
                if cell_in_front.alive {
                    let energy_to_give = self.current_instruction().e.clamp(0.0, self.energy);
                    cell_in_front.energy += energy_to_give;
                    self.energy -= energy_to_give;
                }
            }
            Instruction::AttackCell => {
                if self.energy >= config::ATTACK_REQUIRED_ENERGY && cell_in_front.alive {
                    self.energy -= config::ATTACK_REQUIRED_ENERGY;

                    let taken_energy = f32::min(cell_in_front.energy, config::ATTACK_ENERGY);
                    cell_in_front.energy -= taken_energy;
                    self.energy += taken_energy;
                }
            }
            Instruction::RecycleDeadCell => {
                if cell_in_front.is_dead() {
                    self.energy += cell_in_front.energy;
                    cell_in_front.empty = true;
                }
            }

            Instruction::CheckEnergy => {
                next_instruction = if self.energy > self.current_instruction().e {
                    self.current_instruction().b1
                } else {
                    self.current_instruction().b2
                }
            }

            Instruction::CheckIfDirectedLeft => {
                next_instruction = if let Direction::Left = self.direction {
                    self.current_instruction().b1
                } else {
                    self.current_instruction().b2
                }
            }
            Instruction::CheckIfDirectedRight => {
                next_instruction = if let Direction::Right = self.direction {
                    self.current_instruction().b1
                } else {
                    self.current_instruction().b2
                }
            }
            Instruction::CheckIfDirectedUp => {
                next_instruction = if let Direction::Up = self.direction {
                    self.current_instruction().b1
                } else {
                    self.current_instruction().b2
                }
            }
            Instruction::CheckIfDirectedDown => {
                next_instruction = if let Direction::Down = self.direction {
                    self.current_instruction().b1
                } else {
                    self.current_instruction().b2
                }
            }

            Instruction::CheckIfFacingAliveCell => {
                next_instruction = if cell_in_front.alive {
                    self.current_instruction().b1
                } else {
                    self.current_instruction().b2
                }
            }
            Instruction::CheckIfFacingDeadCell => {
                next_instruction = if cell_in_front.is_dead() {
                    self.current_instruction().b1
                } else {
                    self.current_instruction().b2
                }
            }
            Instruction::CheckIfFacingVoid => {
                next_instruction = if cell_in_front.empty {
                    self.current_instruction().b1
                } else {
                    self.current_instruction().b2
                }
            }

            Instruction::CheckIfFacingRelative => 'b: {
                if !cell_in_front.alive {
                    next_instruction = self.current_instruction().b2;
                    break 'b;
                }

                let mut similar_genes = 0;

                let theirs = &cell_in_front.genome;
                for (i, gene) in theirs.iter().enumerate() {
                    if self.genome[i].instruction == gene.instruction {
                        similar_genes += 1;
                    }
                }

                next_instruction = if similar_genes == config::GENOME_LENGTH {
                    self.current_instruction().b1
                } else {
                    self.current_instruction().b2
                }
            }

            Instruction::MakeChild => 'b: {
                if self.energy < config::REPRODUCTION_REQUIRED_ENERGY && !cell_in_front.empty {
                    next_instruction = self.current_instruction().b2;
                    break 'b;
                }

                let mut child = Bot {
                    x: looking_x,
                    y: looking_y,
                    age: 0,
                    energy: config::START_ENERGY,
                    current_instruction: 0,
                    ..*self
                };

                if rand::thread_rng().gen_bool(config::MUTATION_PERCENT / 100.0) {
                    let gene_to_mutate = rand::thread_rng().gen_range(0..config::GENOME_LENGTH - 1);
                    child.genome[gene_to_mutate as usize].mutate();
                    // Mutate child's color to be slightly different from the parent
                    child.color.mutate(16.0);
                }

                map.set(child.x, child.y, child);
                self.energy -= config::REPRODUCTION_REQUIRED_ENERGY;
                next_instruction = self.current_instruction().b1;
            }

            Instruction::Noop => {}
        }

        // If instruction pointer goes beyond the end of genome, wrap around
        if next_instruction >= config::GENOME_LENGTH {
            next_instruction = 0;
        }
        self.current_instruction = next_instruction;

        self.energy -= config::NOOP_COST;
        // Cell can die of age, or if it has <0 energy
        if self.age > config::CELL_MAX_AGE || self.energy < 0.0 {
            self.alive = false;
        }

        self.age += 1;
    }
}
