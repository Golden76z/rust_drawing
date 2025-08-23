// Struct to create & randomize a new Point
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Method to create a new point with given coordinates
    pub fn new(x: i32, y: i32) -> Self {
        Point {
            x:  x,
            y:  y,
        }
    }

    // Method to create a new Point with random coordinates given the canva size
    pub fn random(width: i32, height: i32) -> Self {
        Point {
            x:  1,
            y:  1,
        }
    }
}