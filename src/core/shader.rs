extern crate glium;

#[derive(Copy, Clone)]
pub struct PTCVertex {
    pub position: [f32; 3],
    pub tex: [f32; 3],
    pub color: [f32; 4],
}

implement_vertex!(PTCVertex, position, tex, color);

pub fn simple_program<T : glium::backend::Facade>(display : &T) -> glium::Program {
  let program = program!(display,
    330 => {
        vertex: "
            #version 330

            uniform mat4 matrix;

            in vec3 position;
            in vec3 tex;
            in vec4 color;

            out vec4 vColor;

            void main() {
                gl_Position = vec4(position, 1.0) * matrix;
                vColor = color;
            }
        ",

        fragment: "
            #version 330

            in vec4 vColor;

            out vec4 f_color;

            void main() {
                f_color = vec4(vColor);
            }
        "
    },
  ).unwrap();
  program
}