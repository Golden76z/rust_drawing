use crate::geometrical_shapes::color::random_color;
use crate::geometrical_shapes::{Drawable, Line, point::Point};
use raster::Color;

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
            color: random_color(),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut raster::Image) {
        // ab -> bc -> cd -> ad
        let ab = Line::new(
            &self.top_left,
            &Point::new(self.bottom_right.x, self.top_left.y),
            &self.color,
        );
        let bc = Line::new(
            &Point::new(self.bottom_right.x, self.top_left.y),
            &self.bottom_right,
            &self.color,
        );
        let cd = Line::new(
            &self.bottom_right,
            &Point::new(self.top_left.x, self.bottom_right.y),
            &self.color,
        );
        let ad: Line = Line::new(
            &Point::new(self.top_left.x, self.bottom_right.y),
            &self.top_left,
            &self.color,
        );

        ab.draw(image);
        bc.draw(image);
        cd.draw(image);
        ad.draw(image);
    }
}

