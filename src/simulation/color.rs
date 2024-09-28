use std::ops::Mul;

use rand::prelude::*;
use serde::{Deserialize, Serialize};

/// 24-bit RGBA color
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Color(u8, u8, u8);

impl From<Color> for macroquad::color::Color {
    fn from(clr: Color) -> Self {
        macroquad::color::Color::from_rgba(clr.0, clr.1, clr.2, 255)
    }
}

// Implement distribution to be able to generate random colors
impl Distribution<Color> for rand::distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        Color::new(rng.gen(), rng.gen(), rng.gen())
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color(
            (self.0 as f64 * rhs) as u8,
            (self.1 as f64 * rhs) as u8,
            (self.2 as f64 * rhs) as u8,
        )
    }
}

impl Color {
    pub const BLACK: Color = Color::new(0, 0, 0);

    /// Create a new color with specified RGB values
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color(r, g, b)
    }

    pub fn r(&self) -> u8 {
        self.0
    }
    pub fn g(&self) -> u8 {
        self.1
    }
    pub fn b(&self) -> u8 {
        self.2
    }

    /// Change a random color component by a random number in range `(-amount..=amount)`
    pub fn mutate(&mut self, amount: f64) {
        let mut rng = thread_rng();

        // Convert the color components to f64 and mutate them,
        // this is to not overflow the original u8 type
        let mut r = self.r() as f64;
        let mut g = self.g() as f64;
        let mut b = self.b() as f64;

        match rng.gen_range(0..=2) {
            0 => r += rng.gen_range(-amount..=amount),
            1 => g += rng.gen_range(-amount..=amount),
            2 => b += rng.gen_range(-amount..=amount),
            _ => {}
        }

        self.0 = r.clamp(0., 255.) as u8;
        self.1 = g.clamp(0., 255.) as u8;
        self.2 = b.clamp(0., 255.) as u8;
    }
}
