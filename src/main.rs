#[macro_use]
extern crate glium;

mod support;

use glium::Surface;
use glium::glutin;
use glium::index::PrimitiveType;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Something {
  x: (u32, u32),
  y: u32
}

fn main() {
  use glium::DisplayBuild;
  use glium::glutin::GlRequest;
  use glium::glutin::GlProfile;
  use glium::glutin::Api;
  use glium::glutin::WindowBuilder;

  // building the display, ie. the main object
  let display = WindowBuilder::new()
                 .with_gl_profile(GlProfile::Core)
                 .with_gl(GlRequest::Specific(Api::OpenGl,(4,0)))
                 .build_glium().unwrap();



  // building the vertex buffer, which contains all the vertices that we will draw
  let vertex_buffer = {
      #[derive(Copy, Clone)]
      struct Vertex {
          position: [f32; 2],
          color: [f32; 3],
      }

      implement_vertex!(Vertex, position, color);

      glium::VertexBuffer::new(&display,
          &[
              Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
              Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
              Vertex { position: [ 0.5, -0.5], color: [1.0, 0.0, 0.0] },
          ]
      ).unwrap()
  };

  // building the index buffer
  let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
                                             &[0u16, 1, 2]).unwrap();

  // compiling shaders and linking them together
    let program = program!(&display,
        330 => {
            vertex: "
                #version 330

                uniform mat4 matrix;

                in vec2 position;
                in vec3 color;

                out vec3 vColor;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 330

                in vec3 vColor;
                out vec4 f_color;

                void main() {
                    f_color = vec4(vColor, 1.0);
                }
            "
        },

    ).unwrap();

   // the main loop
    support::start_loop(|| {
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
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();


        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return support::Action::Stop,
                e => println!("got {:?}", e)
            }
        }

        support::Action::Continue
    });
}
