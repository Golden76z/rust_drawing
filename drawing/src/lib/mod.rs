// lib/mod.rs - This declares all your modules
pub mod point;
pub mod line; 
pub mod circle;
pub mod triangle;
pub mod rectangle;
pub mod traits;
pub mod color;

// Optionally re-export for convenience
pub use point::Point;
pub use line::Line;
pub use circle::Circle;
pub use triangle::Triangle;
pub use rectangle::Rectangle;
pub use traits::*;
pub use color::*;