extern crate gm2;
#[macro_use]
extern crate glium;
extern crate glutin;

use gm2::game;
use gm2::core;

use glutin::Event;

fn main() {
  let window = core::render::build_window();
  let render_state = core::render::render_state(&window);

  // let mut state = game::GameState { tick: 12 };
  // state = game::update(state);

  core::game::start_loop(|| {
    // building the uniforms
    core::render::render(&window, &render_state);

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
