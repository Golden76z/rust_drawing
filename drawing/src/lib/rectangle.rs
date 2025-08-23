use crate::lib::{point::Point, Drawable, Line};
use raster::Color;
use crate::lib::color::{random_color};

pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
    color: Color,
}
impl Rectangle {
    pub fn new(top_left: &Point, bottom_right: &Point) -> Self {
        Rectangle {
            top_left: top_left.clone(),
            bottom_right: bottom_right.clone(),
            color:  random_color(),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut raster::Image) {
        // ab -> bc -> ca
        let ab = Line::new(&self.a, &self.b, &self.color);
        let bc = Line::new(&self.b, &self.c, &self.color);
        let ca = Line::new(&self.a, &self.c, &self.color);
        let ad: Line = Line::new(&self.a, &self.c, &self.color);

        ab.draw(image);
        bc.draw(image);
        ca.draw(image);
    }
}