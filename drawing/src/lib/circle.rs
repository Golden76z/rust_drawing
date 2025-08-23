use raster::Image;

use crate::lib::{Displayable, Drawable, Point};

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn random(width: i32, height: i32) -> Self {
        let center = Point::random(width, height);
        let radius = rand::random_range(1..=1000);
        Circle { center, radius }
    }
}

impl Drawable for Circle {
    // Bressenham here else will have to use trigono my ass
    fn draw(&self, image: &mut Image) {
        let mut x = self.radius;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            image.display(self.center.x + x, self.center.y + y, self.center.color.clone());
            image.display(self.center.x + y, self.center.y + x, self.center.color.clone());
            image.display(self.center.x - y, self.center.y + x, self.center.color.clone());
            image.display(self.center.x - x, self.center.y + y, self.center.color.clone());
            image.display(self.center.x - x, self.center.y - y, self.center.color.clone());
            image.display(self.center.x - y, self.center.y - x, self.center.color.clone());
            image.display(self.center.x + y, self.center.y - x, self.center.color.clone());
            image.display(self.center.x + x, self.center.y - y, self.center.color.clone());

            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            } 
            if err > 0 {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }
}