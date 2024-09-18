use rand::prelude::*;
use serde::{Deserialize, Serialize};

/// 24-bit RGBA color
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Color([u8; 4]);

impl From<Color> for macroquad::color::Color {
    fn from(clr: Color) -> Self {
        macroquad::color::Color::from_rgba(clr.r(), clr.g(), clr.b(), clr.a())
    }
}

impl Color {
    pub const BLACK: Color = Color::new(0, 0, 0);

    /// Create a new color with specified RGB values, and alpha value of 255
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color([r, g, b, 255])
    }

    #[inline(always)]
    pub fn r(&self) -> u8 {
        self.0[0]
    }
    #[inline(always)]
    pub fn g(&self) -> u8 {
        self.0[1]
    }
    #[inline(always)]
    pub fn b(&self) -> u8 {
        self.0[2]
    }
    #[inline(always)]
    pub fn a(&self) -> u8 {
        self.0[3]
    }

    /// Change a random color component by a random number in range `(-amount..=amount)`
    pub fn mutate(&mut self, amount: f64) {
        let mut rng = thread_rng();

        let mut r = self.r() as f64;
        let mut g = self.g() as f64;
        let mut b = self.b() as f64;

        match rng.gen_range(0..=2) {
            0 => r += rng.gen_range(-amount..=amount),
            1 => g += rng.gen_range(-amount..=amount),
            2 => b += rng.gen_range(-amount..=amount),
            _ => {}
        }

        self.0[0] = r.clamp(0.0, 255.0) as u8;
        self.0[1] = g.clamp(0.0, 255.0) as u8;
        self.0[2] = b.clamp(0.0, 255.0) as u8;
    }
}
