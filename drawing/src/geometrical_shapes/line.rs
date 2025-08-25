use crate::geometrical_shapes::{Displayable, Drawable, point::Point, random_color};
use raster::{Color, Image};

// Line define structure used for line
pub struct Line {
    start: Point,
    end: Point,
    color: Color,
}

// impl Line
impl Line {
    pub fn new(a: &Point, b: &Point, color: &Color) -> Self {
        Line {
            start: a.clone(),
            end: b.clone(),
            color: color.clone(),
        }
    }

    pub fn random(w: i32, h: i32) -> Self {
        Line {
            start: Point::random(w, h),
            end: Point::random(w, h),
            color: random_color(),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let mut x0 = self.start.x;
        let mut y0 = self.start.y;
        let x1 = self.end.x;
        let y1 = self.end.y;

        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            image.display(x0, y0, self.color.clone());

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}

