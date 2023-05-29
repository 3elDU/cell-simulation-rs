use num_derive::FromPrimitive;
use num_derive::ToPrimitive;
use num_traits::FromPrimitive;
use rand::prelude::*;
use sdl2::pixels::Color;

use crate::config;
use crate::gene;
use crate::simulation::Simulation;

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
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
    fn apply_direction(&self, x: usize, y: usize) -> (usize, usize) {
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

#[derive(Copy, Clone)]
pub struct Bot {
    pub alive: bool,
    pub empty: bool,

    pub x: usize,
    pub y: usize,
    pub energy: f64,
    pub direction: Direction,
    pub color: Color,
    pub age: usize,

    pub genome: [gene::Gene; config::GENOME_LENGTH],
    current_instruction: usize,
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
    pub fn new_random(x: usize, y: usize) -> Self {
        let mut genome = [gene::Gene::default(); config::GENOME_LENGTH];

        // Generate the genome
        for gene in genome.iter_mut().take(config::GENOME_LENGTH) {
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

            color: Color::RGB(rand::random(), rand::random(), rand::random()),
            genome,
            current_instruction: 0,
        }
    }

    // Generates an empty bot
    pub fn new_empty(x: usize, y: usize) -> Self {
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

    // Whether a bot should update
    pub fn should_update(&self) -> bool {
        self.alive
    }

    // Returns reference to the current instruction
    pub fn current_instruction(&self) -> &gene::Gene {
        &self.genome[self.current_instruction]
    }

    // Whether a bot is a dead cell
    pub fn is_dead(&self) -> bool {
        !self.alive && !self.empty
    }

    // When a mutation happens, child also has slightly different colors from the parent.
    // This function mutates the color.
    fn mutate_color(&mut self) {
        // Convert the color to f64 first, so that it won't overflow
        let mut r = self.color.r as f64;
        let mut g = self.color.g as f64;
        let mut b = self.color.b as f64;

        let mut rng = thread_rng();
        r += rng.gen_range(-16.0..=16.0);
        g += rng.gen_range(-16.0..=16.0);
        b += rng.gen_range(-16.0..=16.0);

        r = r.clamp(127.0, 255.0);
        g = g.clamp(127.0, 255.0);
        b = b.clamp(127.0, 255.0);

        self.color = Color::RGB(r as u8, g as u8, b as u8);
    }

    // Update a bot
    // Bot needs a mutable reference to the simulation to be able to lookup other bots and change their fields
    // Example: Attacking other bots (changing their energy), checking the bot in front
    pub fn update(&mut self, ctx: &mut Simulation) {
        if !self.alive {
            return;
        }

        let mut next_instruction = self.current_instruction + 1;
        let (looking_x, looking_y) = self.direction.apply_direction(self.x, self.y);
        let cell_in_front = ctx.cell_at_mut(looking_x, looking_y).unwrap();

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
                let energy_to_give = self.current_instruction().e.clamp(0.0, self.energy);
                cell_in_front.energy += energy_to_give;
                self.energy -= energy_to_give;
            }
            Instruction::AttackCell => {
                if self.energy >= config::ATTACK_REQUIRED_ENERGY && cell_in_front.alive {
                    self.energy -= config::ATTACK_REQUIRED_ENERGY;

                    // If 'opt' is true, killing the bot in front
                    if self.current_instruction().opt {
                        self.energy += cell_in_front.energy;
                        cell_in_front.alive = false;
                        cell_in_front.empty = true;
                        cell_in_front.energy = 0.0;
                    } else {
                        // Otherwise, taking config::ATTACK_ENERGY% energy from the bot in front
                        let taken_energy = cell_in_front.energy * config::ATTACK_ENERGY;
                        cell_in_front.energy -= taken_energy;
                        self.energy += taken_energy;
                    }
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

            Instruction::CheckIfFacingRelative => {
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

            Instruction::MakeChild => {
                if self.energy > config::REPRODUCTION_REQUIRED_ENERGY && !cell_in_front.alive {
                    let mut child = Bot {
                        x: looking_x,
                        y: looking_y,
                        age: 0,
                        energy: config::START_ENERGY,
                        current_instruction: 0,
                        ..*self
                    };

                    if rand::thread_rng().gen_bool(config::MUTATION_PERCENT / 100.0) {
                        let gene_to_mutate =
                            rand::thread_rng().gen_range(0..config::GENOME_LENGTH - 1);
                        child.genome[gene_to_mutate].mutate();
                        child.mutate_color();
                    }

                    if cell_in_front.is_dead() {
                        child.energy += ctx.cell_at(child.x, child.y).unwrap().energy;
                    }

                    ctx.set_cell_at(child.x, child.y, child);
                }
            }

            Instruction::Noop => {}
        }

        // If instruction pointer goes beyond the end of genome, wrap around
        if next_instruction >= config::GENOME_LENGTH {
            next_instruction %= config::GENOME_LENGTH;
        }
        self.current_instruction = next_instruction;

        // Cell can die of age, or if it has <0 energy
        if self.age > config::CELL_MAX_AGE || self.energy < 0.0 {
            self.alive = false;
        }

        self.age += 1;
    }
}
