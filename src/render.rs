extern crate glium;

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

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

pub fn token_triangle<T : glium::backend::Facade>(display : &T) -> glium::VertexBuffer<Vertex> {
  let vertex_buffer = {
    glium::VertexBuffer::new(display,
      &[
        Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
        Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
        Vertex { position: [ 0.5, -0.5], color: [1.0, 0.0, 0.0] },
      ]
    ).unwrap()
  };

  vertex_buffer
}
pub fn indices<T : glium::backend::Facade>(display : &T) -> glium::IndexBuffer<u16> {
  use glium::index::PrimitiveType;

  glium::IndexBuffer::new(display, PrimitiveType::TrianglesList,
                                             &[0u16, 1, 2]).unwrap()
}

pub fn renderState<T : glium::backend::Facade>(display : &T) -> RenderState {
  use shader;
  RenderState {
    vertices: token_triangle(display),
    indices: indices(display),
    program: shader::simple_program(display)
  }
}

pub struct RenderState {
  pub vertices: glium::VertexBuffer<Vertex>,
  pub indices: glium::IndexBuffer<u16>,
  pub program: glium::Program,
}