#![allow(dead_code)]

mod quads;
mod shader;

extern crate glium;
extern crate image;
extern crate cgmath;

use glium::index;
use glium::Surface;
use core::{Vec3, Mat3, Mat4};
use core::render::quads::GeometryTesselator;
use core::camera;
use cgmath::{Rad};

pub fn build_window() -> glium::backend::glutin_backend::GlutinFacade { //glium::backend::glutin_backend::GlutinFacade
  use glium::DisplayBuild;
  use glium::glutin::GlRequest;
  use glium::glutin::GlProfile;
  use glium::glutin::Api;
  use glium::glutin::WindowBuilder;

  WindowBuilder::new()
   .with_gl_profile(GlProfile::Core)
   .with_gl(GlRequest::Specific(Api::OpenGl,(4,0)))
   .with_depth_buffer(24)
   .build_glium()
   .unwrap()
}

pub fn render_state<F>(display: &F) -> RenderState where F : glium::backend::Facade {
  use std::path::Path;
  use std::f64::consts::PI;

  let image = image::open(&Path::new("img/tiles.png")).unwrap().to_rgba();
  let image_dimensions = image.dimensions();
  let image_raw = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
  let texture_array = glium::texture::Texture2dArray::new(display, vec![image_raw]).unwrap();

  let (width, height) = display.get_context().get_framebuffer_dimensions();

  RenderState {
    program: self::shader::simple_program(display),
    texture: TiledTexture{ texture: texture_array, tiles: 32 },
    camera: camera::Camera {
      at: Vec3::new(8.0, 0.0, 8.0),
      pitch: Rad(-PI / 4.0_f64),
    },
    pixels_per_unit: 16.0, // fixed for a game, really ...
    zoom: 3.0, // moveable
    dimensions: (width, height),
  }
}

pub struct RenderState {
  pub program: glium::Program,
  pub texture: TiledTexture,
  pub camera: camera::Camera,
  pub pixels_per_unit: f64,
  pub zoom : f64,
  pub dimensions: (u32, u32), // derived state, ewwww
}

impl RenderState {
  pub fn view(&self) -> Mat4 {
    camera::view(self.camera.pitch, self.camera.at)
  }

  pub fn projection(&self) -> Mat4 {
    let (width, height) = self.dimensions;
    camera::projection(self.zoom, width, height, self.pixels_per_unit)
  }

  pub fn units_per_pixel(&self) -> f64 {
    1.0 / self.pixels_per_unit
  }

  pub fn view_projection(&self) -> Mat4 {
    self.projection() * self.view()
  }
}

pub struct TiledTexture {
  pub texture: glium::texture::texture2d_array::Texture2dArray,
  pub tiles: u32,
}

pub fn render(display: &glium::Display, rs:&RenderState, time:f64, color: [f32; 4]) {
  let tesselator_scale = Vec3::new(rs.units_per_pixel(), rs.units_per_pixel(), rs.units_per_pixel());

  let mut tesselator = GeometryTesselator::new(tesselator_scale);
  let ground_tile = TextureRegion::at(&rs.texture, 0, 0);
  let man = TextureRegion::at(&rs.texture, 1, 0);
  let man_shadow = TextureRegion::at(&rs.texture, 2, 0);

  for x in 0..16 {
    for z in 0..16 {
      // tesselator.color = [(x as f32) * 1.0 / 16.0, (z as f32) * 1.0 / 16.0, 1.0, 1.0];
      tesselator.draw_floor_tile(&ground_tile, 0, x as f64, 0.0, z as f64, 0.0, false);    
    }
  }

  tesselator.draw_wall_base_anchored_at(&man, 0, Vec3::new(1.5, 0.0, 1.5), 0.0, false);
  tesselator.draw_floor_centre_anchored_at(&man_shadow, 0, Vec3::new(1.5, 0.0, 1.5), 0.01, false);
  
  let vertex_buffer = glium::VertexBuffer::new(display,&tesselator.tesselator.vertices).unwrap();

  let mvp_raw : [[f64; 4]; 4] = rs.view_projection().into();
  let mvp_raw_downsized = down_size_m4(mvp_raw);

  let nearest_neighbour_texture = rs.texture.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest).minify_filter(glium::uniforms::MinifySamplerFilter::Nearest);

  let sun_rotation = Mat3::from_angle_x(Rad(time));
  let sun_direction = Vec3::new(0.0, 1.0, 0.0);
  let adjusted_sun_direction = sun_rotation * sun_direction;
  let adjusted_sun_direction_raw = down_size_v3(adjusted_sun_direction.into());

  let uniforms = uniform! {
    matrix: mvp_raw_downsized,
    u_texture_array: nearest_neighbour_texture,
    u_color: color,
    u_alpha_minimum: 0.05_f32,
    u_sun_direction: adjusted_sun_direction_raw,
  };
  
  let mut target = display.draw();
  target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
  target.draw(&vertex_buffer, &index::NoIndices(index::PrimitiveType::TrianglesList), &rs.program, &uniforms, &draw_params()).unwrap();
  target.finish().unwrap();
}

pub fn draw_params<'a>() -> glium::DrawParameters<'a> {
  let draw_parameters = glium::DrawParameters {
     depth: glium::Depth {
         test: glium::draw_parameters::DepthTest::IfLess,
         write: true,
         .. Default::default()
     },
     blend: glium::Blend::alpha_blending(),
     .. Default::default()
  };
  draw_parameters
}

pub fn down_size_v3(arr: [f64; 3]) -> [f32; 3] {
  let mut out : [f32; 3] = [0.0; 3];
  for a in 0..3 {
    out[a] = arr[a] as f32
  }
  out
}

pub fn down_size_m4(arr: [[f64; 4];4]) -> [[f32; 4]; 4] {
  let mut out : [[f32; 4]; 4] = [[0.0; 4]; 4];
  for a in 0..4 {
    for b in 0..4 {
      out[a][b] = arr[a][b] as f32
    }
  }

  out
}

pub struct TextureRegion {
  pub u_min: u32,
  pub u_max: u32,
  pub v_min: u32,
  pub v_max: u32,
  pub texture_size: u32,
}

impl TextureRegion {
  pub fn at(texture:&TiledTexture, x:u32, y:u32) -> TextureRegion {
    let w: u32 = texture.texture.get_width();
    
    let pixels_per_tile = w / texture.tiles;

    TextureRegion {
      u_min: x * pixels_per_tile,
      u_max: (x + 1) * pixels_per_tile,
      v_min: y * pixels_per_tile,
      v_max: (y + 1) * pixels_per_tile,
      texture_size : w,
    }
  }

  pub fn width(&self) -> u32 {
    self.u_max - self.u_min
  }

  pub fn height(&self) -> u32 {
    self.v_max - self.v_min
  }

  pub fn nu_min(&self) -> f32 {
    (self.u_min as f32) / (self.texture_size as f32)
  }

  pub fn nu_max(&self) -> f32 {
    (self.u_max as f32) / (self.texture_size as f32)
  }

  pub fn nv_min(&self) -> f32 {
    (self.v_min as f32) / (self.texture_size as f32)
  }

  pub fn nv_max(&self) -> f32 {
    (self.v_max as f32) / (self.texture_size as f32)
  }

  pub fn nu_mid(&self) -> f32 {
    (self.nu_min() + self.nu_max()) / 2.0
  }

  pub fn nv_mid(&self) -> f32 {
    (self.nv_min() + self.nv_max()) / 2.0
  }

  pub fn n_width(&self) -> f32 {
    ((self.u_max - self.u_min) as f32) / (self.texture_size as f32)
  }

  pub fn n_height(&self) -> f32 {
    ((self.v_max - self.v_min) as f32) / (self.texture_size as f32)
  }
}
