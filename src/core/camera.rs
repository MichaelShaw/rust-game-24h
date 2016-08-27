extern crate cgmath;

use core::{Mat4, Vec3};
use cgmath::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Camera {
  pub at: Vec3,
  pub pitch: Rad<f64>
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Line {
  pub from: Vec3,
  pub to: Vec3,
}

impl Line {
  pub fn intersects(&self, plane:Plane) -> Option<Vec3> {
    let epsilon = 0.0000001;

    let direction = (self.to - self.from).normalize();
    let denominator = dot(plane.normal, direction);

    if denominator > -epsilon && denominator < epsilon {
      return None
    }
    let numerator = -(dot(plane.normal, self.from) - plane.coefficient);
    let ratio = numerator / denominator;

    if ratio < epsilon {
      None
    } else {
      Some((direction * ratio) + self.from)
    }
  }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Plane {
  pub normal: Vec3,
  pub coefficient: f64,
}

impl Plane {
  pub fn from_origin_normal(origin:Vec3, normal:Vec3) -> Plane {
    Plane {
      normal:normal,
      coefficient: (normal.x * origin.x + normal.y * origin.y + normal.z * origin.z),
    }
  }
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

