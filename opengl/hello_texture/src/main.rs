#[macro_use]
extern crate glium;

use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coord: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coord);

fn main() {
    use glium::glutin;

    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    let shape = vec![
        Vertex {
            position: [-0.5, -0.5],
            tex_coord: [0.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5],
            tex_coord: [1.0, 0.0],
        },
        Vertex {
            position: [0.5, 0.5],
            tex_coord: [1.0, 1.0],
        },
        Vertex {
            position: [0.5, 0.5],
            tex_coord: [1.0, 1.0],
        },
        Vertex {
            position: [-0.5, 0.5],
            tex_coord: [0.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5],
            tex_coord: [0.0, 0.0],
        },
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = std::fs::read_to_string("shaders/vertex.glsl").unwrap();
    let fragment_shader_src = std::fs::read_to_string("shaders/fragment.glsl").unwrap();

    let program =
        glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None)
            .unwrap();

    let image = image::load(
        std::io::Cursor::new(&include_bytes!("../res/hamster.webp")),
        image::ImageFormat::WebP,
    )
    .unwrap()
    .to_rgba8();
    let dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let mut time: f32 = 0.0;
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
            time += 0.0005;

            let mut target = display.draw();
            let uniforms = uniform! {
                matrix: [
                    [ time.cos(), time.sin(), 0.0, 0.0 ],
                    [-time.sin(), time.cos(), 0.0, 0.0 ],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ],
                tex: &texture
            };
            target.clear_color(0.43921568627, 0.50196078431, 0.56470588235, 1.0);

            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();

            target.finish().unwrap();
        }
        _ => return,
    });
}
