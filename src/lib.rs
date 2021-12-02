pub mod camera;
pub mod canvas;
pub mod color;
pub mod intersection;
pub mod light;
pub mod material;
pub mod matrix;
pub mod pattern;
pub mod shapes;
pub mod ray;
pub mod shape;
pub mod transformations;
pub mod tuple;
pub mod world;

pub const EPSILON: f64 = 0.0001;
pub const PI: f64 = std::f64::consts::PI;

pub use crate::shapes::sphere as sphere;
pub use crate::shapes::plane as plane;
