#[macro_use]
extern crate glium;

use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

fn main() {
    use glium::glutin;

    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    let vert1 = Vertex{ position: [ -0.5, 0.5 ], color: [1.0, 0.0, 0.0] };
    let vert2 = Vertex{ position: [ 0.25, -0.5 ], color: [0.0, 1.0, 0.0] };
    let vert3 = Vertex{ position: [ 0.5, 0.5 ], color: [0.0, 0.0, 1.0] };
    let shape = vec![vert1, vert2, vert3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
    #version 140
    in vec2 position;
    in vec3 color;
    out vec3 vertex_color;

    uniform mat4 matrix;

    void main() {
        vertex_color = color;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
    "#;

    let fragment_shader_src = r#"
    #version 140

    in vec3 vertex_color;
    out vec4 color;

    void main() {
        color = vec4(vertex_color, 1.0);
    }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut time: f32 = 0.0;
    event_loop.run(move |loop_event, _, control_flow| {
        match loop_event {
            glutin::event::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                    glutin::event::WindowEvent::Resized(window_size) => {
                        display.gl_window().window().set_inner_size(window_size);
                    },
                    _ => (),
                }
            },
            glutin::event::Event::RedrawEventsCleared => {
                display.gl_window().window().request_redraw();
            },
            glutin::event::Event::RedrawRequested(_) => {
                time += 0.0005;

                let mut target = display.draw();
                let uniforms = uniform!{
                    matrix: [
                        [ time.cos(), time.sin(), 0.0, 0.0 ],
                        [-time.sin(), time.cos(), 0.0, 0.0 ],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0],
                    ]
                };
                target.clear_color(0.43921568627, 0.50196078431, 0.56470588235, 1.0);

                target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();

                target.finish().unwrap();
            },
            _ => return,
        }
    });
}
