#![allow(dead_code)]

extern crate glium;

#[derive(Copy, Clone)]
pub struct PTCVertex {
    pub position: [f32; 3],
    pub tex_coord: [f32; 3],
    pub color: [f32; 4],
    pub normal: [f32; 3],
}

implement_vertex!(PTCVertex, position, tex_coord, color, normal);

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
            in vec3 normal;
            
            out vec4 v_color;
            out vec3 v_tex_coord;
            out vec3 v_normal;

            void main() {
                gl_Position = matrix * vec4(position, 1.0);
                v_color = color * u_color;
                v_tex_coord = tex_coord;
                v_normal = normal; // no model matrix, so no transform required
            }
        ",

        fragment: "
            #version 330

            uniform sampler2DArray u_texture_array;
            uniform float u_alpha_minimum;
            uniform vec3 u_sun_direction;

            in vec4 v_color;
            in vec3 v_tex_coord;
            in vec3 v_normal;

            out vec4 f_color;

            void main() {
                vec3 to_light = normalize(vec3(0.0, 1.0, 0.0));
                float light = clamp(dot(v_normal, u_sun_direction), 0.2, 1.0);

                vec4 albedo_colour = texture(u_texture_array, v_tex_coord) * v_color;
                
                vec4 final_colour = albedo_colour; // * light;
                final_colour.a = albedo_colour.a; // ignore light's alpha

                if(final_colour.a < u_alpha_minimum) {
                    discard;
                }
                f_color = final_colour;
            }
        "
    },
  ).unwrap();
  program
}