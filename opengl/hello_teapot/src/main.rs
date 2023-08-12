#[macro_use]
extern crate glium;

mod teapot;

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

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();

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

    // the direction of the light
    let light = [-1.0, 0.4, 0.9f32];

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
            let uniforms = uniform! {
                matrix: [
                    [0.01, 0.0, 0.0, 0.0 ],
                    [0.0, 0.01, 0.0, 0.0 ],
                    [0.0, 0.0, 0.01, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32],
                ],
                tex: &texture,
                u_light: light,
            };
            target.clear_color_and_depth((0.43921568627, 0.50196078431, 0.56470588235, 1.0), 1.0);

            let params = glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            };

            target
                .draw(
                    (&positions, &normals),
                    &indices,
                    &program,
                    &uniforms,
                    &params,
                )
                .unwrap();

            target.finish().unwrap();
        }
        _ => return,
    });
}
