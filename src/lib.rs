//! # kalman_rs
//! 
//! `kalman_rs` is a collection of utilities for using the kalman filter
//! 

#[macro_use]
mod macros;

pub mod config;
pub mod geometry;
#[macro_use]
pub mod filter;
pub mod error;
pub mod generate_data;

pub use geometry::rectangle::Rectangle;
pub use geometry::trapezoid::Trapezoid;
pub use geometry::traits as sensor_traits;
