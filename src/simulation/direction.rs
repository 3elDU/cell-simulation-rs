use crate::Config;

use rand_derive2::RandGen;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, RandGen)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    // Applies directional movement to given coordinates
    pub fn apply_direction(&self, x: usize, y: usize, config: &Config) -> (usize, usize) {
        match self {
            Self::Left => {
                if x == 0 {
                    (config.width - 1, y)
                } else {
                    (x - 1, y)
                }
            }
            Self::Right => {
                if x == config.width - 1 {
                    (0, y)
                } else {
                    (x + 1, y)
                }
            }
            Self::Up => {
                if y == 0 {
                    (x, 0)
                } else {
                    (x, y - 1)
                }
            }
            Self::Down => {
                if y == config.height - 1 {
                    (x, config.height - 1)
                } else {
                    (x, y + 1)
                }
            }
        }
    }

    // 'Rotates' direction to the left, returning a new one
    pub fn left(&self) -> Self {
        match self {
            Self::Down => Direction::Right,
            Self::Right => Direction::Up,
            Self::Up => Direction::Left,
            Self::Left => Direction::Down,
        }
    }

    // 'Rotates' direction to the right, returning a new one
    pub fn right(&self) -> Self {
        match self {
            Self::Left => Direction::Up,
            Self::Up => Direction::Right,
            Self::Right => Direction::Down,
            Self::Down => Direction::Left,
        }
    }
}
