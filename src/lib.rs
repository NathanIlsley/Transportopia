mod texture;

use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::{
    application::ApplicationHandler, event::*, event_loop::{ActiveEventLoop, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::Window
};

// Struct to describe vertex data
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        // A description of how the data is structured in the vertex buffer
        wgpu::VertexBufferLayout {
            // Size of the data for each vertex
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            // Whether each element of the array represents per-vertex data or per-instance data
            step_mode: wgpu::VertexStepMode::Vertex,
            // Structure of each vertexes' data
            attributes: &[
                wgpu::VertexAttribute {
                    // First attribute starts at the beginning of the vertex data
                    offset: 0,
                    // This attribute will correspond to @location(0) in the shader
                    shader_location: 0,
                    // Format as Vec3<f32>
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    // Second attribute starts with an offset equal to the size of the first attribute
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    // This attribute will correspond to @location(1) in the shader
                    shader_location: 1,
                    // Format as Vec3<f32>
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
    }
}

const VERTICES: &[Vertex] = &[
    // Changed
    Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0], },
    Vertex { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [0.5, 0.5, 0.0], tex_coords: [1.0, 0.0], },
    Vertex { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 1.0], },

    Vertex { position: [-0.0, -1.0, 0.0], tex_coords: [0.0, 1.0], },
    Vertex { position: [-0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0], },
    Vertex { position: [1.0, -1.0, 0.0], tex_coords: [1.0, 1.0], },
];

const INDICES: &[u16] = &[
    0, 1, 2,
    2, 3, 0,

    4, 5, 6,
    6, 7, 4,
];

// This will store the state of our game
pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    window: Arc<Window>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,
}

impl State {
    async fn new(window: Arc<Window>) -> anyhow::Result<State> {
        // Get size of window
        let size = window.inner_size();

        // Create wgpu instance used to create adapter and surface
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            ..Default::default()
        });

        // Create surface to draw to using the window Arc pointer, allowing wgpu to talk to winit
        let surface = instance.create_surface(window.clone()).unwrap();

        // Create adapter to interface with GPU hardware
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        // Create device to use GPU resources and queue to submit commands to GPU
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        // Get the capabilities of the surface
        let surface_caps = surface.get_capabilities(&adapter);
        // Loop through the available formats for the surface and find one that is sRGB
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        // Create surface configuration using the surface format and size of window as well as
        // Fifo present mode which is the only one that is guaranteed to be supported (also allows v-sync)
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        // Load grass texture as byte array
        let diffuse_bytes = include_bytes!("..\\assets\\grass_0.png");
        // Create texture from bytes
        let diffuse_texture = texture::Texture::from_bytes(
            &device,
            &queue,
            diffuse_bytes,
            "diffuse_texture",
        ).unwrap();

        // Create a bind group layout to describe the shader bindings 
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    // Create a binding at 0 for the texture visible to the fragment shader
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    // Create a binding at 1 for the sampler visible to the fragment shader
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });
        // Create a bind group to hold the actual bindings for the shader
        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    // Attach the diffuse_texture_view to binding 0 of the shader
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    // Attach the diffuse_sampler to binding 1 of the shader
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );
        
        // Include the shader code
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        // Create a render pipeline layout
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        // Create render pipeline to describe the render process
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState{  
                module: &shader,
                entry_point: Some("vs_main"), // Use vs_main as the vertex shader
                buffers: &[
                    // Get a description of the vertex buffer
                    Vertex::desc(),
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"), // Use fs_main as the fragment shader
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw, // Vertices in cw orientation are facing the camera
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1, // No multisampling
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        // Create vertex buffer from VERTICES slice
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        // Create index buffer from INDICES slice
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        // Record the number of indices in the index buffer
        let num_indices = INDICES.len() as u32;

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            diffuse_texture,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        // Update size of window and configure surface
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        // Check which key was pressed
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }

    fn update(&mut self) {
        // remove `todo!()`
    }
    
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // Request winit to redraw the window
        self.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }
        
        // Get the next texture to be presented by the swapchain
        let output = self.surface.get_current_texture()?;

        // Create a TextureView type from the Texture type which specifies how the 
        // data in the Texture should be read and used by the GPU (e.g. format, dimension, mipmap levels)
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create a CommandEncoder which is used to create a CommandBuffer which is then added to the queue
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            // Begin the render pass
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                // A slice of all the different textures that the fragment shader's output will be written to
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        depth_slice: None,
                        resolve_target: None,
                        // List of operations to perform at the start (load) and end of the render pass
                        ops: wgpu::Operations {
                            // On load, clear the screen
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            // At the end of the render pass, store the result onto the texture
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // Use the render pipeline specified earlier
            render_pass.set_pipeline(&self.render_pipeline);
            // Use the bind group created earlier for the texture
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            // Create a vertex buffer at slot 0 using all (..) of self.vertex_buffer
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            // Create an index buffer using all (..) of self.index_buffer
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            // Draw the vertices of the vertex buffer as triangles using the indices in the index buffer
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        // Add the CommandBuffer created by the CommandEncoder onto the queue
        // (Submit will accept anything that implements IntoIter)
        self.queue.submit(std::iter::once(encoder.finish()));
        // Display the texture
        output.present();

        Ok(())
    }
}

pub struct App {
    state: Option<State>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: None,
        }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Use default values for window
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        // Set an Arc pointer to point to the new window
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        // Use pollster to wait until the State struct has been created
        self.state = Some(pollster::block_on(State::new(window)).unwrap());
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        // Winit allows a new object of type <T> to be passed by EventLoopProxy::send_event which runs this
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        // If self.state is None, do nothing
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        // Handle the event
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                // If a redraw is requested, update and render state and handle errors with render
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
            }
            // Send keyboard input event to state to be handled
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    // Using env_logger to improve error messages from wgpu
    env_logger::init();

    // Create event loop for winit
    let event_loop = EventLoop::with_user_event().build()?;
    // Create App object
    let mut app = App::new();
    // Run the application
    event_loop.run_app(&mut app)?;

    Ok(())
}