use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use rand::prelude::*;

use crate::config;

#[derive(Debug, FromPrimitive, ToPrimitive, Copy, Clone, PartialEq)]
// Enum for all possible instructions
pub enum Instruction {
    // No operation. Speaks for itself
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
    MakeChild,
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::Noop
    }
}

impl Instruction {
    // Generates a random instruction
    pub fn new_random() -> Self {
        let idx = thread_rng().gen_range(0..=(Instruction::MakeChild as usize));
        FromPrimitive::from_usize(idx).unwrap()
    }
}

#[derive(FromPrimitive, ToPrimitive)]
// Used in Gene::mutate() to determine which field to mutate
enum ThingToMutate {
    Instruction,
    Opt,
    B1,
    B2,
}

impl ThingToMutate {
    pub fn new_random() -> Self {
        let idx = thread_rng().gen_range(0..=(ThingToMutate::B2 as usize));
        FromPrimitive::from_usize(idx).unwrap()
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Gene {
    pub instruction: Instruction,

    // Option. Changes how some instructions behave.
    pub opt: bool,
    // Energy. Used in some instructions as amount of energy to check for, to give, etc.
    pub e: f64,
    // Branch 1. Instruction pointer, used in conditional instrucions (Check*, Jmp*)
    pub b1: usize,
    // Branch 2. Instruction pointer, used in conditional instructions (Check*, Jmp*)
    pub b2: usize,
}

impl Gene {
    // Create a new, randomly generated gene
    pub fn new_random() -> Self {
        let mut rng = thread_rng();
        Gene {
            instruction: Instruction::new_random(),
            opt: rng.gen(),
            e: rng.gen_range(0.0..config::REPRODUCTION_REQUIRED_ENERGY * 2.0),
            b1: rng.gen_range(0..config::GENOME_LENGTH),
            b2: rng.gen_range(0..config::GENOME_LENGTH),
        }
    }

    // Mutate one of gene's fields randomly
    pub fn mutate(&mut self) {
        let mut rng = thread_rng();
        match ThingToMutate::new_random() {
            ThingToMutate::Instruction => self.instruction = Instruction::new_random(),
            ThingToMutate::Opt => self.opt = rng.gen(),
            ThingToMutate::B1 => self.b1 = rng.gen_range(0..config::GENOME_LENGTH),
            ThingToMutate::B2 => self.b2 = rng.gen_range(0..config::GENOME_LENGTH),
        };
    }
}
