use crate::lib::{point::Point, random_color};
use crate::lib::traits::*;
use crate::lib::line::Line;
use raster::{Color};

pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
    color: Color,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Triangle {
        Triangle {
            a: a.clone(),
            b: b.clone(),
            c: c.clone(),
            color: random_color(),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut raster::Image) {
        // ab -> bc -> ca
        let ab = Line::new(&self.a, &self.b, &self.color);
        let bc = Line::new(&self.b, &self.c, &self.color);
        let ca = Line::new(&self.a, &self.c, &self.color);

        ab.draw(image);
        bc.draw(image);
        ca.draw(image);
    }
}