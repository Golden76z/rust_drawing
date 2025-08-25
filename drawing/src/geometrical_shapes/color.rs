use rand::Rng;
use raster::Color;

pub fn random_color() -> Color {
    let mut rng = rand::rng();
    Color::rgb(rng.random(), rng.random(), rng.random())
}