extern crate gm2;
#[macro_use]
extern crate glium;
extern crate glutin;
extern crate cgmath;

use gm2::core;
use gm2::core::input;

use glutin::Event;

fn main() {
  let window = core::render::build_window();
  let mut render_state = core::render::render_state(&window);
  let mut input_state = core::input::InputState::default();
  
  // let mut state = game::GameState { tick: 12 };
  // state = game::update(state);
  let color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

  let mut time = 0.0_f64;

  core::game::start_loop(|| {
    time = time + (1.0 / 60.0);
    // let cyclical_time = (time % 8.0) / 8.0;
    // println!("cyclical time -> {}", cyclical_time);
    
    render_state.dimensions = window.get_framebuffer_dimensions();
    // render_state.camera.pitch = Rad(cyclical_time);

    core::render::render(&window, &render_state, time, color);

    // polling and handling the events received by the window
    let evs : Vec<glutin::Event> = window.poll_events().collect();

    let new_input_state = input::produce(&input_state, &evs);
    if input_state != new_input_state {
      println!("input state -> {:?}", new_input_state);
    }
    input_state = new_input_state;
    
    for event in evs {
        match event {
            Event::Closed | Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Escape)) => return core::game::Action::Stop,
            _ => (),// println!("got {:?}", e)
        }
    }

    core::game::Action::Continue
  });
}
