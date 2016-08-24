#![allow(dead_code)]

extern crate glium;

#[derive(Copy, Clone)]
pub struct PTCVertex {
    pub position: [f32; 3],
    pub tex_coord: [f32; 3],
    pub color: [f32; 4],
}

implement_vertex!(PTCVertex, position, tex_coord, color);

pub fn simple_program<T>(display : &T) -> glium::Program where T : glium::backend::Facade {
  let program = program!(display,
    330 => {
        vertex: "
            #version 330

            uniform mat4 matrix;
            uniform vec4 u_color;

            in vec3 position;
            in vec3 tex_coord;
            in vec4 color;

            out vec4 v_color;
            out vec3 v_tex_coord;

            void main() {
                gl_Position = vec4(position, 1.0) * matrix;
                v_color = color * u_color;
                v_tex_coord = tex_coord;
            }
        ",

        fragment: "
            #version 330

            uniform sampler2DArray u_texture_array;

            in vec4 v_color;
            in vec3 v_tex_coord;

            out vec4 f_color;

            void main() {
                vec4 t_colour = texture(u_texture_array, v_tex_coord);
                f_color = t_colour * v_color;
            }
        "
    },
  ).unwrap();
  program
}