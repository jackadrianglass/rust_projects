use bytemuck;
use wgpu::util::DeviceExt;
use winit::window::Window;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::NoUninit, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        // `array_stride` is the width of the data type that you're passing to the gpu.
        // If you were to iterate through the the data, you need to know how many bytes to
        // jump before you get to the next index. This is usually the size of the vertex
        // data structure that you're passing in
        //
        // The `step_mode` describes when the gpu should increment the index (e.g. per vertex
        // or per instance). This might be useful if you want to draw many of the same thing
        // but at different positions and rotations
        //
        // The `VertexAttribute`s are the descriptions of the vertex elements that your are passing
        // into the buffer. This is generally a one to one mapping to the struct that we're passing
        // in
        //
        // The `offset` is the offset from the start of the vertex that you need to go to access
        // the field of the vertex. e.g. if you wanted to access the first field, you would pass in
        // 0. If you wanted the second field, you would pass in the size of the first field in
        // bytes so that you reach the start of the second field
        //
        // The `location` tells the shader the location where to store the attribute. This is
        // basically a way to how you can access your data from the gpu end
        //
        // The `format` attribute tells the gpu the shape of the attribute (basically how to
        // interpret the bits and bytes that you're being handed). This corresponds to the data
        // types that you use in the shader program
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3, // maps to vec3<f32>
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
        // The above is the expanded version which is quite verbose. Could do this instead
        /*
        impl Vertex {
            const ATTRIBS: [wgpu::VertexAttribute; 2] =
                wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

            fn desc() -> wgpu::VertexBufferLayout<'static> {
                use std::mem;

                wgpu::VertexBufferLayout {
                    array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &Self::ATTRIBS,
                }
            }
        }
        */
    }
}

// If you have types in your vertex that don't implement Pod and Zeroable, you
// can add these lines to get it to work. It's just telling the type system that
// this is okay
/*
unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}
*/

// This is the vertices of a pentagon
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [1.0, 0.0, 0.0] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.0, 1.0, 0.0] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.0, 0.0, 1.0] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], color: [1.0, 0.0, 0.0] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.0, 1.0, 0.0] }, // E
];

// This is what the vertex buffer would be if you didn't have an index buffer
/*
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] },

    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] },

    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] },
];
*/

// This is the index buffer. This allows you to use the same data in multiple triangles.
// Otherwise, you'd have to pass in the same data multiple times. On very large complicated
// meshes, this would dramatically increase the size of the data. It's probably excessive
// for a simple shape but it's good to practice this stuff
#[rustfmt::skip]
const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,

    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_vertices: u32,
}

impl State {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,   // make sure that this isn't 0
            height: size.height, // make sure that this isn't 0
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_vertices: INDICES.len() as u32,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, _event: &WindowEvent) -> bool {
        // don't have any inputs that we want to capture at this time
        false
    }

    fn update(&mut self) {
        // don't have anything to update at this time
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?; // this is the render target

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            // IMPORTANT! Need to set the vertex buffer otherwise your app will crash
            // since the gpu is operating on no data
            render_pass.set_vertex_buffer(
                // This is the buffer slot that you're setting. This is so that you
                // can use multiple different vertex buffers in your render pipeline
                0,
                // This is the buffer that you want to use. It takes in a slice so you
                // can pass in just parts of your buffer or the entire thing
                self.vertex_buffer.slice(..));
            // Note that we're using set_index_buffer not set_index_buffers (plural). You can only
            // set one index buffer at a time
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            // When you use an index buffer, you need call `draw_indexed` rather than `draw` since
            // the latter ignores the vertex buffer and assumes that you pass in groups of vertices
            // based on the primitive type. `draw` would panic since there's not enough vertices
            render_pass.draw_indexed(0..self.num_vertices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            state.window().request_redraw();
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    });
}
