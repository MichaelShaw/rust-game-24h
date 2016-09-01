extern crate glium;
extern crate image;

use std::path::Path;

pub struct TiledTexture {
    pub texture: glium::texture::texture2d_array::Texture2dArray,
    pub tiles: u32,
}

pub fn load_tiled_texture<F>(display: &F, paths: &[&Path], tiles: u32) -> TiledTexture where F : glium::backend::Facade {
    use glium::texture;

    let mut images : Vec<texture::RawImage2d<u8>> = vec![];
    // let v: Vec<i32> = vec![];
    // pub type RgbaImage = ImageBuffer<Rgba<u8>, Vec<u8>>;
    // RawImage2d<'a, T>
    for path in paths {
        let image = image::open(path).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image_raw = texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        images.push(image_raw);
    }

    let texture_array = texture::Texture2dArray::new(display, images).unwrap();
    TiledTexture {
        texture: texture_array,
        tiles: tiles,
    }
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
