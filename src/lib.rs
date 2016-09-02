#![crate_name="gm2"]
#![allow(dead_code)]

#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

pub mod render;
pub mod camera;
pub mod input;
pub mod geometry;
pub mod game {
    pub mod simple;
}
pub mod spring;


pub type Color = [f32; 4];

pub type Vec2i = cgmath::Vector2<i32>;
pub type Vec3i = cgmath::Vector3<i32>;

pub type Vec3 = cgmath::Vector3<f64>;
pub type Vec3f = cgmath::Vector3<f32>;

pub type Vec4 = cgmath::Vector4<f64>;

pub type Mat3 = cgmath::Matrix3<f64>;
pub type Mat4 = cgmath::Matrix4<f64>;

pub fn round_down(f:f64) -> i32 {
    if f < 0.0 {
        f as i32 - 1
    } else {
        f as i32
    }
}

pub fn round_down_v3(v:Vec3) -> Vec3i {
    Vec3i::new(round_down(v.x), round_down(v.y), round_down(v.z))
}