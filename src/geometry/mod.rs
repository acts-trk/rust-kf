//! Sensor structs and traits used in the kalman filter steps

pub mod rectangle;
pub mod traits;
pub mod trapezoid;
pub mod utils;

pub use rectangle::Rectangle;
pub use trapezoid::Trapezoid;
