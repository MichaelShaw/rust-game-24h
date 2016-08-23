extern crate gm2;
#[macro_use]
extern crate glium;
extern crate glutin;

use gm2::game;
use gm2::render;
use gm2::shader;

use glutin::Event;
use glium::Surface;

fn main() {
  let window = render::build_window();
  let rs = render::renderState(&window);

  let pr = shader::simple_program(&window);

  let mut state = game::GameState { tick: 12 };
  state = game::update(state);

  game::start_loop(|| {
    // building the uniforms
    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ]
    };

    // drawing a frame
    let mut target = window.draw();
    target.clear_color(0.0, 0.0, 0.0, 0.0);
    target.draw(&rs.vertices, &rs.indices, &rs.program, &uniforms, &Default::default()).unwrap();
    target.finish().unwrap();

    // polling and handling the events received by the window
    for event in window.poll_events() {
        match event {
            glutin::Event::Closed => return game::Action::Stop,
            glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Escape)) => return game::Action::Stop,
            e => println!("got {:?}", e)
        }
    }

    game::Action::Continue
  });
}
