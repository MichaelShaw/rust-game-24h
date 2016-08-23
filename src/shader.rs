extern crate glium;

pub fn simple_program<T : glium::backend::Facade>(display : &T) -> glium::Program {
  let program = program!(display,
    330 => {
        vertex: "
            #version 330
            uniform mat4 matrix;
            in vec2 position;
            in vec3 color;
            out vec3 vColor;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0) * matrix;
                vColor = color;
            }
        ",

        fragment: "
            #version 330
            in vec3 vColor;
            out vec4 f_color;
            void main() {
                f_color = vec4(vColor, 1.0);
            }
        "
    },
  ).unwrap();
  program
}