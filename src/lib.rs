#![crate_name="gm2"]
#![allow(dead_code)]

#[macro_use]
extern crate glium;

pub mod core {
  pub mod camera;
  pub mod render;
  pub mod shader;
  pub mod game;
}
pub mod game;
pub mod input;
