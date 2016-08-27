extern crate cgmath;

use core::{Mat4, Vec3};
use cgmath::Rad;

pub struct Camera {
  pub at: Vec3,
  pub pitch: Rad<f64>
}

pub fn view(pitch: Rad<f64>, at: Vec3) -> Mat4 {
  Mat4::from_angle_x(pitch) * Mat4::from_translation(at * -1.0)
}

pub fn projection(zoom:f64, width:u32, height:u32, pixels_per_unit: f64) -> Mat4 {
  let effective_width = (width as f64) / (zoom * pixels_per_unit);
  let effective_height = (height as f64) / (zoom * pixels_per_unit) / (2.0_f64).sqrt(); // ; // adjust for 45 degree downward viewing angle
  let half_width = effective_width / 2.0;
  let half_height = effective_height / 2.0;

  cgmath::ortho(-half_width, half_width, -half_height, half_height, -100.0, 100.0)
}