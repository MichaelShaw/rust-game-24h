#![allow(dead_code)]

mod quads;
mod shader;

extern crate glium;
extern crate image;
extern crate cgmath;

use glium::index;
use glium::Surface;

pub fn build_window() -> glium::backend::glutin_backend::GlutinFacade { //glium::backend::glutin_backend::GlutinFacade
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

pub fn render_state<F>(display: &F) -> RenderState where F : glium::backend::Facade {
  use std::path::Path;

  let image = image::open(&Path::new("img/small.scene.png")).unwrap().to_rgba();
  let image_dimensions = image.dimensions();
  let image_raw = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
  let texture_array = glium::texture::Texture2dArray::new(display, vec![image_raw]).unwrap();

  RenderState {
    program: self::shader::simple_program(display),
    texture: texture_array,
  }
}

pub struct RenderState {
  pub program: glium::Program,
  pub texture: glium::texture::texture2d_array::Texture2dArray,
}

pub fn render(display: &glium::Display, rs:&RenderState, color: [f32; 4]) {
  use core::render::shader::PTCVertex;

  let mut quad_tesselator : quads::QuadTesselator<PTCVertex> = quads::QuadTesselator { vertices: Vec::new() };
  quad_tesselator.add_quad(
    [
      PTCVertex { position: [-0.5, -0.5, 0.0], tex_coord: [0.0, 0.0, 0.0], color: [0.5, 1.0, 0.5, 1.0] },
      PTCVertex { position: [-0.5,  0.5, 0.0], tex_coord: [0.0, 1.0, 0.0], color: [0.25, 0.25, 1.0, 1.0] },
      PTCVertex { position: [0.5,   0.5, 0.0], tex_coord: [1.0, 1.0, 0.0], color: [1.0, 0.5, 0.5, 1.0] },
      PTCVertex { position: [0.5,  -0.5, 0.0], tex_coord: [1.0, 0.0, 0.0], color: [0.5, 0.1, 1.0, 1.0] }
    ]
  );

  let vertex_buffer = glium::VertexBuffer::new(display,&quad_tesselator.vertices).unwrap();

  let uniforms = uniform! {
    matrix: [
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0f32]
    ],
    u_texture_array: &rs.texture,
    u_color: color,
  };

  // drawing a frame
  let mut target = display.draw();
  target.clear_color(0.0, 0.0, 0.0, 0.0);
  target.draw(&vertex_buffer, &index::NoIndices(index::PrimitiveType::TrianglesList), &rs.program, &uniforms, &Default::default()).unwrap();
  target.finish().unwrap();

  ()
}