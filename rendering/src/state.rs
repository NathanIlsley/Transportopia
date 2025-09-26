use crate::camera;
use crate::sprite;
use crate::instance;
use crate::vertex;
use crate::texture;

use std::sync::Arc;
use winit::window::Window;
use winit::keyboard::KeyCode;
use wgpu::util::DeviceExt;
use std::time::Instant;
use std::collections::HashMap;

const VERTICES: &[vertex::Vertex] = &[
    vertex::Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0], },
    vertex::Vertex { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 0.0], },
    vertex::Vertex { position: [0.5, 0.5, 0.0], tex_coords: [1.0, 0.0], },
    vertex::Vertex { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 1.0], },
];

const INDICES: &[u16] = &[
    0, 1, 2,
    2, 3, 0,
];

// This will store the state of our game
pub(crate) struct State<C: camera::Camera, S: sprite::Sprite> {
    last_instant: Instant,
    sprite_key_interests: HashMap<KeyCode, Vec<usize>>,
    camera_key_interests: Vec<KeyCode>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    pub(crate) window: Arc<Window>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    texture_bind_groups: Vec<wgpu::BindGroup>,
    sprites: Vec<S>,
    instance_buffer: wgpu::Buffer,
    camera: C,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
}

impl<C: camera::Camera, S: sprite::Sprite> State<C, S> {
    pub(crate) async fn new(window: Arc<Window>, camera: C, sprites: Vec<S>, textures: &[&[u8]]) -> anyhow::Result<State<C, S>> {
        let last_instant = Instant::now();
        
        let mut sprite_key_interests: HashMap<KeyCode, Vec<usize>> = HashMap::new();
        for (i, sprite) in sprites.iter().enumerate() {
            for key in sprite.interested_keys() {
                sprite_key_interests.entry(key).or_insert_with(Vec::new).push(i);
            }
        }

        let camera_key_interests: Vec<KeyCode> = camera.interested_keys();

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
        
        // Create a bind group layout to describe the shader bindings for textures
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
            }
        );

        // Include the shader code
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        
        
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
        
        // Create a bind group for each texture and sampler
        let mut texture_bind_groups = Vec::new();
        textures.iter().for_each(|texture| {
            let texture = texture::Texture::from_bytes(
                &device,
                &queue,
                texture,
                "diffuse_texture",
            ).unwrap();
            let bind_group = device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    layout: &texture_bind_group_layout,
                    entries: &[
                        // Attach the diffuse_texture_view to binding 0 of the shader
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&texture.view),
                        },
                        // Attach the diffuse_sampler to binding 1 of the shader
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&texture.sampler),
                        }
                        ],
                        label: Some("diffuse_bind_group"),
                    }
                );
                texture_bind_groups.push(bind_group);
            });

            // Get data for each instance
            let mut instance_data = Vec::new();
            for sprite in &sprites {
                instance_data.push(sprite.get_instance());
            }
            
        // Create instance buffer that contains the instance data
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                // Use this buffer in the vertex shader
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );
        
        // Create a camera buffer to hold the camera matrix
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&camera.get_matrix()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        // Create a bind group layout to describe the shader bindings for the camera
        let camera_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
                label: Some("camera_bind_group_layout"),
            }
        );
        // Bind the camera buffer to the camera bind group
        let camera_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &camera_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_buffer.as_entire_binding(),
                    }
                ],
                label: Some("camera_bind_group"),
            }
        );

        // Create a render pipeline layout
        let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &texture_bind_group_layout,
                &camera_bind_group_layout,
                ],
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
                vertex::Vertex::desc(),
                // Get a description of the instance buffer
                instance::Instance::desc(),
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

        Ok(Self {
            last_instant,
            sprite_key_interests,
            camera_key_interests,
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
            vertex_buffer,
            index_buffer,
            num_indices,
            texture_bind_groups,
            sprites,
            instance_buffer,
            camera,
            camera_buffer,
            camera_bind_group,
            render_pipeline,
        })
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        // Update size of window and configure surface
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub(crate) fn update(&mut self) {
        // Get time between frames
        let current_instant = Instant::now();
        let delta_time = current_instant.duration_since(self.last_instant).as_secs_f64();
        self.last_instant = current_instant;

        // Update camera
        self.camera.update(delta_time);

        // Update camera buffer with new camera matrix
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&self.camera.get_matrix())
        );

        // Update each sprite
        for sprite in &mut self.sprites {
            sprite.update(delta_time);
        }

        // Update instance buffer with any changed instance data
        let mut instance_data = Vec::new();
        let mut change_start: usize = 0;
        for (i, sprite) in self.sprites.iter_mut().enumerate() {
            if sprite.changed() {
                sprite.change_handled();
                instance_data.push(sprite.get_instance());
            } else if change_start != i {
                self.queue.write_buffer(
                    &self.instance_buffer,
                    (change_start * std::mem::size_of::<instance::Instance>()) as wgpu::BufferAddress,
                    bytemuck::cast_slice(&instance_data[change_start..i])
                );
                change_start = i + 1;
                instance_data = Vec::new();
            } else {
                change_start += 1;
            }
        }
        // Write any remaining changed instance data to the buffer
        if change_start != self.sprites.len() {
            self.queue.write_buffer(
                &self.instance_buffer,
                (change_start * std::mem::size_of::<instance::Instance>()) as wgpu::BufferAddress,
                bytemuck::cast_slice(&instance_data[change_start..])
            );
        }

    }

    pub(crate) fn handle_key(&mut self, key: winit::keyboard::KeyCode, pressed: bool) {
        // Update each sprite that is interested in this key event
        if let Some(sprite_indices) = self.sprite_key_interests.get(&key) {
            for &i in sprite_indices {
                self.sprites[i].key_event(key, pressed);
            }
        }
        // Update camera if it is interested in this key event
        if self.camera.interested_keys().contains(&key) {
            self.camera.key_event(key, pressed);
        }
    }
    
    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
            render_pass.set_bind_group(0, &self.texture_bind_groups[0], &[]);
            // Use the bind group created earlier for the camera
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            // Create a vertex buffer at slot 0 using all (..) of self.vertex_buffer
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            // Create an index buffer using all (..) of self.index_buffer
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            // Draw the vertices of the vertex buffer as triangles using the indices in the index buffer
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.sprites.len() as u32);
        }

        // Add the CommandBuffer created by the CommandEncoder onto the queue
        // (Submit will accept anything that implements IntoIter)
        self.queue.submit(std::iter::once(encoder.finish()));
        // Display the texture
        output.present();

        Ok(())
    }
}