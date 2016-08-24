extern crate glium;

use core::shader::PTCVertex;
use glium::index;
use glium::Surface;

pub fn build_window() -> glium::backend::glutin_backend::GlutinFacade {
  use glium::DisplayBuild;
  use glium::glutin::GlRequest;
  use glium::glutin::GlProfile;
  use glium::glutin::Api;
  use glium::glutin::WindowBuilder;

  WindowBuilder::new()
   .with_gl_profile(GlProfile::Core)
   .with_gl(GlRequest::Specific(Api::OpenGl,(4,0)))
   .build_glium()
   .unwrap()
}

pub fn indices(display : &glium::Display) -> glium::IndexBuffer<u16> {
  use glium::index::PrimitiveType;

  glium::IndexBuffer::new(display, PrimitiveType::TrianglesList,
                                             &[0u16, 1, 2]).unwrap()
}

pub fn renderState(display : &glium::Display) -> RenderState {
  use core::shader;
  RenderState {
    program: shader::simple_program(display)
  }
}

pub struct RenderState {
  pub program: glium::Program,
}

pub fn render(display : &glium::Display , rs:&RenderState) -> () {
  let nullTex : [f32; 3] = [0.0, 0.0, 0.0];
  let vertex_buffer = {
    glium::VertexBuffer::new(display,
      &[
        PTCVertex { position: [-0.5, -0.5, 0.0], tex:nullTex, color: [0.0, 1.0, 0.0, 1.0] },
        PTCVertex { position: [ 0.0,  0.5, 0.0], tex:nullTex, color: [0.0, 0.0, 1.0, 1.0] },
        PTCVertex { position: [ 0.5, -0.5, 0.0], tex:nullTex, color: [1.0, 0.0, 0.0, 1.0] },
      ]
    ).unwrap()
  };

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

  target.draw(&vertex_buffer, &index::NoIndices(index::PrimitiveType::TrianglesList), &rs.program, &uniforms, &Default::default()).unwrap();
  target.finish().unwrap();

  ()
}