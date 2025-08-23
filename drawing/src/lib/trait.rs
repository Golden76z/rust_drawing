use raster::{Color, Image};

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}
impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}