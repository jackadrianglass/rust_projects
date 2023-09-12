#![allow(dead_code)]
#[macro_use]
extern crate glium;

use glium::Surface;
use glam::Vec2;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

const CUBE: [Vec2; 4] = [
    Vec2{ x: 0.5, y: 0.5 },
    Vec2{ x: -0.5, y: 0.5 },
    Vec2{ x: 0.5, y: -0.5 },
    Vec2{ x: -0.5, y: -0.5 },
];

const IDX: [u32; 6] = [0, 1, 2, 1, 3, 2];

fn main() {
    use glium::glutin;

    let positions = CUBE.iter().map(|v| Vertex{position: v.to_array() }).collect::<Vec<_>>();

    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display, &positions).unwrap();
    let indices =
        glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &IDX)
            .unwrap();

    let vertex_shader_src = std::fs::read_to_string("shaders/vertex.glsl").unwrap();
    let fragment_shader_src = std::fs::read_to_string("shaders/fragment.glsl").unwrap();
    let program =
        glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None)
            .unwrap();

    event_loop.run(move |loop_event, _, control_flow| match loop_event {
        glutin::event::Event::WindowEvent { event, .. } => match event {
            glutin::event::WindowEvent::CloseRequested => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
                return;
            }
            glutin::event::WindowEvent::Resized(window_size) => {
                display.gl_window().window().set_inner_size(window_size);
            }
            _ => (),
        },
        glutin::event::Event::RedrawEventsCleared => {
            display.gl_window().window().request_redraw();
        }
        glutin::event::Event::RedrawRequested(_) => {
            let mut target = display.draw();

            target.clear_color_and_depth((0.43921568627, 0.50196078431, 0.56470588235, 1.0), 1.0);

            let params = glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                ..Default::default()
            };

            target.draw(&positions, &indices, &program, &glium::uniforms::EmptyUniforms, &params).unwrap();

            target.finish().unwrap();
        }
        _ => return,
    });
}
