use winit::window::Window;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: Window,

    render_pipeline: wgpu::RenderPipeline,
}

impl State {
    // Creating some of the wgpu types requires async code
    async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        // Main purpose for the most part is to create the Adapter and Surface
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        //
        // This is the thing that the graphics card will draw to
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        // This is the handle to our grafics card
        // Can use this to create our device and queue later
        // !! This won't work for every device !!
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                // Can pick between battery saving mode or high performance
                power_preference: wgpu::PowerPreference::default(),
                // Tells the adapter to pick an adapter that's compatible with the provided surface
                compatible_surface: Some(&surface),
                // forces wgpu to fallback to an adapter that will work for all hardware
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // Note that you can also request a specific adapter
        /*
        ```rs
        let adapter = instance
            .enumerate_adapters(wgpu::Backends::all())
            .find(|adapter| {
                // Check if this adapter supports our surface
                adapter.is_surface_supported(&surface)
            }) .unwrap()
        ```
        */

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

        //> The next little bit defines how the surface creates the underlying surface texture

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
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

        //> The next little bit is to load the shader progam

        // Loads the shader pipeline into the program
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
                // You can specify which function is the entry point
                entry_point: "vs_main",
                // Tells what type of vertices that we want to pass to the gpu
                // Since this round all of the data is in the shader itself, this
                // will be left empty
                buffers: &[],
            },
            // Fragment is optional. We will need it if we want to store color data to the surface
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                // This tells wgpu what color targets to setup. Only need one for the surface
                // Use the surface format so that copying to it is easy
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    // This says to replace the old data with new data
                    blend: Some(wgpu::BlendState::REPLACE),
                    // This says to write all colours to the surface
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                // This is how the vertices are interpreted
                // Three vertices to a triangle
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                // How do determine what is the front of the surface face
                front_face: wgpu::FrontFace::Ccw,
                // This the gpu how to surface cull
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            // Not using stencils at this time. This is for rendering to a portion of the
            // surface
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                // Number of samples being used. This is complicated so don't worry about
                // it just yet
                count: 1,
                // Specifies which samples should be active. We're activating all of them
                mask: !0,
                // This is related to anti aliasing. Also not going to be covered here
                alpha_to_coverage_enabled: false,
            },
            // How many array layers the render attachments can have. Won't be doing this
            // yet so don't worry about it
            multiview: None,
        });
        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
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

        // This is the actual clearing of the screen (finally!)
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    Some(wgpu::RenderPassColorAttachment {
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
                    }),
                ],
                depth_stencil_attachment: None,
            });
            // Specify that the render pass should use the pipeline that we just created
            render_pass.set_pipeline(&self.render_pipeline);
            // Specify that we're drawing 3 vertices and one instance
            render_pass.draw(0..3, 0..1);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

// todo: Use the `input()` method to capture cursor events and update the clear colour
//      hint: may want to use WindowEvent::CursorMoved
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
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
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
                        // new_inner_size is &&mut so we have to dereference it twice
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    });
}
