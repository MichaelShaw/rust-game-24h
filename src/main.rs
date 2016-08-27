extern crate gm2;
#[macro_use]
extern crate glium;
extern crate glutin;
extern crate cgmath;

use gm2::game;
use gm2::core;

use glutin::Event;
use gm2::core::{Vec3, Mat3, Mat4};
use gm2::core::camera;
use std::f64::consts::PI;
use cgmath::Rad;
use glium::Surface;

fn main() {
  let window = core::render::build_window();
  let mut render_state = core::render::render_state(&window);

  // let mut state = game::GameState { tick: 12 };
  // state = game::update(state);
  let mut color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

  let mut time = 0.0_f64;

  core::game::start_loop(|| {
    // color[1] = (color[1] + 0.01) % 1.0;
    time = time + (1.0 / 60.0);

    let cyclical_time = (time % 8.0) / 8.0;
    // println!("cyclical time -> {}", cyclical_time);
    // {
      // render_state.dimensions = window.get_context().get_framebuffer_dimensions();
    // }
    render_state.camera.pitch = Rad(cyclical_time);

    // render_state.view = camera::view(Rad(0.25 * PI), Vec3::new(8.0, 0.0, 8.0));

    core::render::render(&window, &render_state, time, color);

    // polling and handling the events received by the window
    for event in window.poll_events() {
        match event {
            Event::Closed | Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Escape)) => return core::game::Action::Stop,
            e => println!("got {:?}", e)
        }
    }

    core::game::Action::Continue
  });
}
