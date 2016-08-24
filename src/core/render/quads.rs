extern crate cgmath;

use core::render::shader::PTCVertex;

pub type Vec3 = cgmath::Vector3<f32>;

pub struct QuadTesselator<T> {
  pub vertices: Vec<T>,
}

impl<T : Copy> QuadTesselator<T> {
  pub fn add_quad(&mut self, ts:[T; 4]) {
    for ele in ts.iter() {
      self.vertices.push(ele.clone());
    }
    let va = self.vertices[self.vertices.len() - 4];
    self.vertices.push(va);
    let vb = self.vertices[self.vertices.len() - 3];
    self.vertices.push(vb);
  }

  pub fn clear(&mut self) {
    self.vertices.clear();
  }
}

struct GeometryTesselator {
  pub tesselator: QuadTesselator<PTCVertex>,
  pub scale: Vec3, // scale ... translating pixel coord to real world coords
  pub color: [f32; 4],
}

impl GeometryTesselator {
  // anchor is near x/z coord
  pub fn draw_floor_tile(&mut self, tr:TextureRegion, layer:u32, ax:f32, y:f32, az:f32, depth_adjust:f32, flip:bool) {
    let layer_f = layer as f32;
    let ww = (tr.width() as f32) * self.scale.x;
    let dw = (tr.height() as f32) * self.scale.z;

    let nu_left = if flip { tr.nu_max() } else { tr.nu_min() };
    let nu_right = if flip { tr.nu_min() } else { tr.nu_max() };

    self.tesselator.add_quad([
      PTCVertex { position: [ax,      y + depth_adjust, az + dw + depth_adjust], tex_coord: [nu_left , tr.nv_min(), layer_f], color: self.color },
      PTCVertex { position: [ax + ww, y + depth_adjust, az + dw + depth_adjust], tex_coord: [nu_right, tr.nv_min(), layer_f], color: self.color },
      PTCVertex { position: [ax + ww, y + depth_adjust, az + depth_adjust     ], tex_coord: [nu_right, tr.nv_max(), layer_f], color: self.color },
      PTCVertex { position: [ax,      y + depth_adjust, az + depth_adjust     ], tex_coord: [nu_left , tr.nv_max(), layer_f], color: self.color }
    ]);
  }

  // anchor is centre of tile
  pub fn draw_floor_centre_anchored_at(&mut self, tr:TextureRegion, layer:u32, v:Vec3, depth_adjust:f32, flip:bool)  {
    self.draw_floor_centre_anchored(tr, layer, v.x, v.y, v.z, depth_adjust, flip)
  }

  pub fn draw_floor_centre_anchored(&mut self, tr:TextureRegion, layer:u32, ax:f32, y:f32, az:f32, depth_adjust:f32, flip:bool) {
    let layer_f = layer as f32;

    let hww = (tr.width() as f32) * self.scale.x / 2.0;
    let hdw = (tr.height() as f32) * self.scale.z / 2.0;

    let nu_left = if flip { tr.nu_max() } else { tr.nu_min() };
    let nu_right = if flip { tr.nu_min() } else { tr.nu_max() };

    self.tesselator.add_quad([
      PTCVertex { position: [ax - hww, y + depth_adjust, az + hdw + depth_adjust], tex_coord: [nu_left , tr.nv_min(), layer_f], color: self.color },
      PTCVertex { position: [ax + hww, y + depth_adjust, az + hdw + depth_adjust], tex_coord: [nu_right, tr.nv_min(), layer_f], color: self.color },
      PTCVertex { position: [ax + hww, y + depth_adjust, az - hdw + depth_adjust], tex_coord: [nu_right, tr.nv_max(), layer_f], color: self.color },
      PTCVertex { position: [ax + hww, y + depth_adjust, az - hdw + depth_adjust], tex_coord: [nu_left , tr.nv_max(), layer_f], color: self.color }
    ]);
  }

  pub fn draw_floor_centre_anchored_rotated_at(&mut self, tr:TextureRegion, layer:u32, v:Vec3, theta:f32, depth_adjust:f32)  {
    self.draw_floor_centre_anchored_rotated(tr, layer, v.x, v.y, v.z, theta, depth_adjust)
  }

  pub fn draw_floor_centre_anchored_rotated(&mut self, tr:TextureRegion, layer:u32, ax:f32, y:f32, az:f32, theta:f32, depth_adjust:f32) {
    use cgmath::{Rad, Matrix3, Point3, Transform};

    let layer_f = layer as f32;

    let hww = (tr.width() as f32) * self.scale.x / 2.0;
    let hdw = (tr.height() as f32) * self.scale.z / 2.0;

    let rr : Rad<_> = Rad(theta).into();
    let rot : Matrix3<f32> = Matrix3::from_angle_y(rr);

    let p0 = rot.transform_point(Point3::new(- hww, 0.0, hdw));
    let p1 = rot.transform_point(Point3::new(hww,   0.0, hdw));
    let p2 = rot.transform_point(Point3::new(hww,   0.0, - hdw));
    let p3 = rot.transform_point(Point3::new(- hww, 0.0, - hdw));

    let xx = ax;
    let yy = y + depth_adjust;
    let zz = az + depth_adjust;


    self.tesselator.add_quad([
      PTCVertex { position: [p0.x + xx, p0.y + yy + depth_adjust, p0.z + zz + depth_adjust], tex_coord: [tr.nu_min(), tr.nv_min(), layer_f], color: self.color },
      PTCVertex { position: [p1.x + xx, p1.y + yy + depth_adjust, p1.z + zz + depth_adjust], tex_coord: [tr.nu_max(), tr.nv_min(), layer_f], color: self.color },
      PTCVertex { position: [p2.x + xx, p2.y + yy + depth_adjust, p2.z + zz + depth_adjust], tex_coord: [tr.nu_max(), tr.nv_max(), layer_f], color: self.color },
      PTCVertex { position: [p3.x + xx, p3.y + yy + depth_adjust, p3.z + zz + depth_adjust], tex_coord: [tr.nu_min(), tr.nv_max(), layer_f], color: self.color }
    ]);
  }

  pub fn draw_wall_base_anchored_at(&mut self, tr:TextureRegion, layer:u32, v:Vec3, depth_adjust:f32, flip:bool) {
    self.draw_wall_base_anchored(tr, layer, v.x, v.y, v.z, depth_adjust, flip)
  }

  pub fn draw_wall_base_anchored(&mut self, tr:TextureRegion, layer:u32, ax:f32, ay:f32, z:f32, depth_adjust:f32, flip:bool) {
    let layer_f = layer as f32;

    let hww = (tr.width() as f32) * self.scale.x / 2.0;
    let hhw = (tr.height() as f32) * self.scale.y;

    let nu_left = if flip { tr.nu_max() } else { tr.nu_min() };
    let nu_right = if flip { tr.nu_min() } else { tr.nu_max() };

    self.tesselator.add_quad([
      PTCVertex { position: [ax - hww, ay + depth_adjust,       z + depth_adjust], tex_coord: [nu_left , tr.nv_min(), layer_f], color: self.color },
      PTCVertex { position: [ax + hww, ay + depth_adjust,       z + depth_adjust], tex_coord: [nu_right, tr.nv_min(), layer_f], color: self.color },
      PTCVertex { position: [ax + hww, ay + hhw + depth_adjust, z + depth_adjust], tex_coord: [nu_right, tr.nv_max(), layer_f], color: self.color },
      PTCVertex { position: [ax - hww, ay + hhw + depth_adjust, z + depth_adjust], tex_coord: [nu_left , tr.nv_max(), layer_f], color: self.color }
    ]);
  }

  pub fn draw_wall_centre_anchored_at(&mut self, tr:TextureRegion, layer:u32, v:Vec3, depth_adjust:f32, flip:bool) {
    self.draw_wall_centre_anchored(tr, layer, v.x, v.y, v.z, depth_adjust, flip)
  }

  pub fn draw_wall_centre_anchored(&mut self, tr:TextureRegion, layer:u32, ax:f32, ay:f32, z:f32, depth_adjust:f32, flip:bool) {
    let layer_f = layer as f32;

    let hww = (tr.width() as f32) * self.scale.x / 2.0;
    let hhw = (tr.height() as f32) * self.scale.y / 2.0;

    let nu_left = if flip { tr.nu_max() } else { tr.nu_min() };
    let nu_right = if flip { tr.nu_min() } else { tr.nu_max() };

    self.tesselator.add_quad([
      PTCVertex { position: [ax - hww, ay - hhw + depth_adjust, z + depth_adjust], tex_coord: [nu_left , tr.nv_min(), layer_f], color: self.color },
      PTCVertex { position: [ax + hww, ay - hhw + depth_adjust, z + depth_adjust], tex_coord: [nu_right, tr.nv_min(), layer_f], color: self.color },
      PTCVertex { position: [ax + hww, ay + hhw + depth_adjust, z + depth_adjust], tex_coord: [nu_right, tr.nv_max(), layer_f], color: self.color },
      PTCVertex { position: [ax - hww, ay + hhw + depth_adjust, z + depth_adjust], tex_coord: [nu_left , tr.nv_max(), layer_f], color: self.color }
    ]);
  }
}


struct TextureRegion {
  pub u_min: u32,
  pub u_max: u32,
  pub v_min: u32,
  pub v_max: u32,
  pub texture_size: u32,
}

impl TextureRegion {
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
