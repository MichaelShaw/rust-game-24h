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

            in vec3 position;
            in vec3 tex_coord;
            in vec4 color;

            out vec4 vColor;
            out vec3 vTexCoord;

            void main() {
                gl_Position = vec4(position, 1.0) * matrix;
                vColor = color;
                vTexCoord = tex_coord;
            }
        ",

        fragment: "
            #version 330

            uniform sampler2DArray u_texture_array;
            uniform vec4 u_color;

            in vec4 vColor;
            in vec3 vTexCoord;

            out vec4 f_color;

            void main() {
                vec4 tColour = texture(u_texture_array, vTexCoord);
                f_color = tColour * vColor * u_color;
            }
        "
    },
  ).unwrap();
  program
}