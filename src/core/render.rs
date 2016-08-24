#![allow(dead_code)]

extern crate glium;
extern crate image;

use core::shader::PTCVertex;
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

pub fn render_state<F>(display : &F) -> RenderState where F : glium::backend::Facade {
  use core::shader;
  use std::path::Path;

  let image = image::open(&Path::new("img/small.scene.png")).unwrap().to_rgba();
  let image_dimensions = image.dimensions();
  let image_raw = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
  let images = vec![image_raw];
  let texture_array = glium::texture::Texture2dArray::new(display, images).unwrap();

  RenderState {
    program: shader::simple_program(display),
    texture: texture_array,
  }
}

pub struct RenderState {
  pub program: glium::Program,
  pub texture: glium::texture::texture2d_array::Texture2dArray,
}

pub struct QuadTesselator<T> {
  vertices: Vec<T>,
  n: u32,
}

impl<T : Copy> QuadTesselator<T> {
  // fn addMultiple(&mut self, t:[T]) {
  //   // for ele in t {
  //   //   self.add(self, ele)
  //   // }
  // }

  fn add(&mut self, t:T) {
    self.vertices.push(t);
    self.n += 1;
    if self.n % 4 == 0 {
      let va = self.vertices[self.vertices.len() - 4];
      self.vertices.push(va);

      let vb = self.vertices[self.vertices.len() - 3];
      self.vertices.push(vb);
    }
  }

  fn clear(&mut self) {
    self.vertices.clear();
    self.n = 0;
  }
}

pub fn render(display : &glium::Display , rs:&RenderState) -> () {
  let mut quad_tesselator : QuadTesselator<PTCVertex> = QuadTesselator { vertices: Vec::new(), n: 0};
  quad_tesselator.add(PTCVertex { position: [-0.5, -0.5, 0.0], tex_coord: [0.0, 0.0, 0.0], color: [0.0, 1.0, 0.0, 1.0] });
  quad_tesselator.add(PTCVertex { position: [-0.5,  0.5, 0.0], tex_coord: [0.0, 1.0, 0.0], color: [0.0, 0.0, 1.0, 1.0] });
  quad_tesselator.add(PTCVertex { position: [0.5,  0.5, 0.0], tex_coord: [1.0, 1.0, 0.0], color: [0.0, 0.0, 1.0, 1.0] });
  quad_tesselator.add(PTCVertex { position: [0.5,  -0.5, 0.0], tex_coord: [1.0, 0.0, 0.0], color: [0.0, 0.0, 1.0, 1.0] });

  let vertex_buffer = glium::VertexBuffer::new(display,&quad_tesselator.vertices).unwrap();

  let uniforms = uniform! {
    matrix: [
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0f32]
    ],
    textureArray: &rs.texture
  };

  // drawing a frame
  let mut target = display.draw();
  target.clear_color(0.0, 0.0, 0.0, 0.0);

  target.draw(&vertex_buffer, &index::NoIndices(index::PrimitiveType::TrianglesList), &rs.program, &uniforms, &Default::default()).unwrap();
  target.finish().unwrap();

  ()
}