#![allow(dead_code)]

extern crate glium;
extern crate cgmath;

pub mod quads;
pub mod shader;
pub mod texture;

pub fn build_window() -> glium::Display { 
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

pub fn round_down(f:f64) -> i32 {
    if f < 0.0 {
        f as i32 - 1
    } else {
        f as i32
    }
}

pub fn translucent_draw_params<'a>() -> glium::DrawParameters<'a> {
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

pub fn opaque_draw_params<'a>() -> glium::DrawParameters<'a> {
    let draw_parameters = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
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
