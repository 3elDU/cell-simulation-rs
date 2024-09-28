use rand::prelude::*;
use rand_derive2::RandGen;
use serde::{Deserialize, Serialize};

use crate::Config;

use super::config;

#[derive(Default, Debug, RandGen, Copy, Clone, PartialEq, Serialize, Deserialize)]
// Enum for all possible instructions
pub enum Instruction {
    // No operation. Speaks for itself
    #[default]
    Noop,

    // Bots have four directions they can face: Left, Right, Up, Down
    // Turns the bot left
    TurnLeft,
    // Turns the bot right
    TurnRight,
    // Moves forward in the bot's direction
    MoveForwards,

    // Makes energy through photosynthesis
    Photosynthesis,
    // Gives instruction.e% energy to cell in front
    GiveEnergy,

    // Attacks cell in front, taking 50% of energy from it (can be configured)
    // If instruction.opt is true, kills the cell in front
    AttackCell,
    // Recycles dead cell in front, taking all energy from it
    RecycleDeadCell,

    // Checks if energy is higher than instruction.b1, then jumps to B1, otherwise jumps to B2
    CheckEnergy,

    // If cell is facing this direction, jumps to B1, otherwise to B2
    CheckIfDirectedLeft,
    CheckIfDirectedRight,
    CheckIfDirectedUp,
    CheckIfDirectedDown,

    // If bot is facing alive cell, jumps to B1, otherwise to B2
    CheckIfFacingAliveCell,
    // If bot is facing dead cell, jumps to B1, otherwise to B2
    CheckIfFacingDeadCell,
    // If bot is facing void, jumps to B1, otherwise to B2
    CheckIfFacingVoid,
    // If bot is facing it's relative, jumps to B1, otherwise to B2
    // 'Relative' is a cell that has all the genes the same.
    // Only instructions are checked, other fields are ignored
    CheckIfFacingRelative,

    // Reproduces. A certain minimum amount of energy is required to reproduced, can be configured.
    // If a child was made successfully, jumps to B1, otherwise to B2
    MakeChild,
}

// Used in Gene::mutate() to determine which field to mutate
#[derive(RandGen)]
enum ThingToMutate {
    Instruction,
    Option,
    Energy,
    Branch,
    BranchAlt,
}

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Gene {
    pub instruction: Instruction,

    // Option. Changes how some instructions behave.
    pub option: bool,
    // Energy. Used in some instructions as amount of energy to check for, to give, etc.
    pub energy: f32,
    // Branch 1. Instruction pointer, used in conditional instrucions (Check*, Jmp*)
    pub branch: u8,
    // Branch 2. Instruction pointer, used in conditional instructions (Check*, Jmp*)
    pub branch_alt: u8,
}

impl Gene {
    // Create a new, randomly generated gene
    pub fn new_random(config: &Config) -> Self {
        let mut rng = thread_rng();
        Gene {
            instruction: Instruction::generate_random(),
            option: rng.gen(),
            energy: rng.gen_range(0.0..config.reproduction_required_energy * 2.0),
            branch: rng.gen_range(0..config::GENOME_LENGTH),
            branch_alt: rng.gen_range(0..config::GENOME_LENGTH),
        }
    }

    // Mutate one of gene's fields randomly
    pub fn mutate(&mut self, config: &Config) {
        let mut rng = thread_rng();
        match ThingToMutate::generate_random() {
            ThingToMutate::Instruction => self.instruction = Instruction::generate_random(),
            ThingToMutate::Option => self.option = rng.gen(),
            ThingToMutate::Energy => {
                self.energy = rng.gen_range(0.0..config.reproduction_required_energy * 2.0)
            }
            ThingToMutate::Branch => self.branch = rng.gen_range(0..config::GENOME_LENGTH),
            ThingToMutate::BranchAlt => self.branch_alt = rng.gen_range(0..config::GENOME_LENGTH),
        };
    }
}
