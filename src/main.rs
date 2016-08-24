extern crate gm2;
#[macro_use]
extern crate glium;
extern crate glutin;

use gm2::game;
use gm2::core;

use glutin::Event;
use glium::Surface;

fn main() {
  let window = core::render::build_window();

  let rs = core::render::renderState(&window);

  let pr = core::shader::simple_program(&window);

  let mut state = game::GameState { tick: 12 };
  state = game::update(state);

  core::game::start_loop(|| {
    // building the uniforms
    core::render::render(&window, &rs);

    // polling and handling the events received by the window
    for event in window.poll_events() {
        match event {
            glutin::Event::Closed | glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Escape)) => return core::game::Action::Stop,
            e => println!("got {:?}", e)
        }
    }

    core::game::Action::Continue
  });
}
