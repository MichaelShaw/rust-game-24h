extern crate cgmath;

use super::*;


pub fn smooth_3d(from:Vec3, to:Vec3, velocity:Vec3, smooth_time:f64, time_delta:f64) -> (Vec3, Vec3) {
    let omega = 2.0 / smooth_time;
    let x = omega * time_delta;
    let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
    let change = to - from;

    let tmp = (velocity + change * omega) * time_delta;

    let new_velocity = (velocity - (tmp * omega)) * exp;

    ((change + tmp) * exp + to, new_velocity)
} 

pub fn smooth_1d(from:f64, to:f64, velocity:f64, smooth_time:f64, time_delta:f64) -> (f64, f64) {
    let omega = 2.0 / smooth_time;
    let x = omega * time_delta;
    let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
    let change = from - to;

    let tmp = (velocity + omega * change) * time_delta;

    let new_velocity = (velocity - omega * tmp) * exp;

    (to + (change + tmp) * exp, new_velocity)
}