use crate::geometrical_shapes::color::random_color;
use crate::geometrical_shapes::{Displayable, Drawable};
use rand::Rng;
use raster::Color;

// Struct to create & randomize a new Point
#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl Point {
    // Method to create a new point with given coordinates
    pub fn new(x: i32, y: i32) -> Self {
        Point {
            x: x,
            y: y,
            color: random_color(),
        }
    }

    // Method to create a new Point with random coordinates given the canva size
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::rng();
        let x: i32 = rng.random_range(0..width);
        let y: i32 = rng.random_range(0..height);

        Point {
            x: x,
            y: y,
            color: random_color(),
        }
    }
}

// Implementing draw functionality for the Point
impl Drawable for Point {
    fn draw(&self, image: &mut raster::Image) {
        image.display(self.x, self.y, self.color.clone());
    }
}
