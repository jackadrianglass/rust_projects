#![allow(dead_code)]
#[macro_use]
extern crate glium;

use glam::{Vec3, Vec4, Mat4, Quat};
use glium::Surface;
use rand::prelude::*;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    colour: [f32; 3],
}
implement_vertex!(Vertex, position, colour);

fn random_colour(rng: &mut ThreadRng) -> [f32; 3] {
    [rng.gen(), rng.gen(), rng.gen()]
}

const CUBE: [Vec3; 8] = [
    Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
    Vec3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    },
    Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    },
    Vec3 {
        x: 1.0,
        y: 0.0,
        z: 1.0,
    },
    Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    },
    Vec3 {
        x: 1.0,
        y: 1.0,
        z: 0.0,
    },
    Vec3 {
        x: 0.0,
        y: 1.0,
        z: 1.0,
    },
    Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    },
];

const IDX: [u32; 36] = [
    0, 1, 2, 1, 2, 3, 4, 5, 6, 5, 6, 7, 0, 4, 5, 4, 5, 1, 0, 4, 6, 4, 6, 2, 1, 3, 5, 5, 7, 3, 2, 3,
    6, 6, 7, 3,
];

fn main() {
    use glium::glutin;
    let mut rng = rand::thread_rng();

    let positions = CUBE
        .iter()
        .map(|v| Vertex {
            position: v.to_array(),
            colour: random_colour(&mut rng),
        })
        .collect::<Vec<_>>();

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

    let mut time = 0.0;
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
                backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
                ..Default::default()
            };

            let matrix = Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_vec4(Vec4::new(time, 0.0, 0.0, 1.0).normalize()),
                Vec3::new(-0.25, -0.25, -1.0),
                );
            time += 0.1;

            target
                .draw(
                    &positions,
                    &indices,
                    &program,
                    &uniform!(matrix: matrix.to_cols_array_2d()),
                    &params,
                )
                .unwrap();

            target.finish().unwrap();
        }
        _ => return,
    });
}
