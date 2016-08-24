#![crate_name="gm2"]
#![allow(dead_code)]

#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

pub mod core {
  pub mod camera;
  pub mod render;
  pub mod game;
}
pub mod game;
pub mod input;

pub type Vec3 = cgmath::Vector3<f64>;
pub type Vec3f = cgmath::Vector3<f32>;