use raster::Image;

mod lib;
use lib::{Point, Line, Triangle, Rectangle, Circle, Drawable};

fn main() {
    let mut image = Image::blank(1000, 1000);

    Line::random(image.width, image.height).draw(&mut image);

    Point::random(image.width, image.height).draw(&mut image);

    let rectangle = Rectangle::new(&Point::new(150, 150), &Point::new(50, 50));
    rectangle.draw(&mut image);

    let triangle = Triangle::new (
            &Point::new(500, 500),
            &Point::new(250, 700),
            &Point::new(700, 800),
    );
    triangle.draw(&mut image);

    for _ in 1..50 {
        Circle::random(image.width, image.height).draw(&mut image);
    }

    raster::save(&image, "image.png").unwrap();
}