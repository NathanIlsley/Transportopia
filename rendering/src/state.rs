use crate::camera;
use crate::sprite;
use crate::instance;
use crate::vertex;
use crate::system;

use std::sync::Arc;
use anyhow::anyhow;
use wgpu::BindGroup;
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
    system: system::System,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    texture_bind_groups: Vec<(wgpu::BindGroup, (usize, usize))>,
    sprites: Vec<S>,
    instance_buffer: wgpu::Buffer,
    camera: C,
    render_pipeline: wgpu::RenderPipeline,
}

impl<C: camera::Camera, S: sprite::Sprite + PartialEq> State<C, S> {
    pub(crate) async fn new(window: Arc<Window>, mut camera: C, sprites: Vec<S>) -> anyhow::Result<State<C, S>> {
        // Record the time at which the last frame was drawn
        let last_instant = Instant::now();
        
        // Create a map of keys to the indices of sprites that are interested in those keys
        let mut sprite_key_interests: HashMap<KeyCode, Vec<usize>> = HashMap::new();
        for (i, sprite) in sprites.iter().enumerate() {
            for key in sprite.interested_keys() {
                sprite_key_interests.entry(key).or_insert_with(Vec::new).push(i);
            }
        }
        
        // Create the system struct to handle wgpu stuff
        let system = system::System::new(window).await?;

        // Include the shader code
        let shader = system.device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        
        // Create vertex buffer from VERTICES slice
        let vertex_buffer = system.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        // Create index buffer from INDICES slice
        let index_buffer = system.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        
        // Record the number of indices in the index buffer
        let num_indices = INDICES.len() as u32;
        
        // Loop through the sprites and work out what indexes of instances correspond to what bind group 
        let mut texture_bind_groups: Vec<(BindGroup, (usize, usize))> = vec![];
        if sprites.len() < 1 {
            return Err(anyhow!("Must have at least one sprite"));
        }
        texture_bind_groups.push((sprites[0].texture().get_bind_group(&system), (0, 1)));
        if sprites.len() != 1 {
            for (i, s) in sprites[1..].iter().enumerate() {
                if i != 0 && *s != sprites[i - 1] {
                    texture_bind_groups.last_mut().unwrap().1.1 = i + 1;
                    texture_bind_groups.push((s.texture().get_bind_group(&system), (i + 1, i + 2)));
                }
            };
        }

        // println!("{}, {}", &texture_bind_groups[0].1.0, &texture_bind_groups[0].1.1);
        // println!("{}, {}", &texture_bind_groups[1].1.0, &texture_bind_groups[1].1.1);
        // println!("{}", &sprites.len());

        // Get data for each instance
        let mut instance_data = Vec::new();
        for sprite in &sprites {
            instance_data.push(sprite.get_instance());
        }
            
        // Create instance buffer that contains the instance data
        let instance_buffer = system.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                // Use this buffer in the vertex shader
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );
        
        // Initialize the camera buffer
        camera.init_buffer(camera::CameraBuffer::new(&system, &camera));

        // Create a render pipeline layout
        let render_pipeline_layout =
        system.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &system.texture_bind_group_layout,
                &camera.buffer().as_ref().unwrap().bind_group_layout,
                ],
            push_constant_ranges: &[],
        });
        // Create render pipeline to describe the render process
        let render_pipeline = system.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                    format: system.config.format,
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
            system,
            vertex_buffer,
            index_buffer,
            num_indices,
            texture_bind_groups,
            sprites,
            instance_buffer,
            camera,
            render_pipeline,
        })
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        // Update size of window and configure surface
        if width > 0 && height > 0 {
            self.system.config.width = width;
            self.system.config.height = height;
            self.system.surface.configure(&self.system.device, &self.system.config);
            self.system.is_surface_configured = true;
        }
    }

    pub(crate) fn get_window_size(&self) -> winit::dpi::PhysicalSize<u32> {self.system.size}

    pub(crate) fn update(&mut self) {
        // Get time between frames
        let current_instant = Instant::now();
        let delta_time = current_instant.duration_since(self.last_instant).as_secs_f64();
        self.last_instant = current_instant;

        // Update camera
        self.camera.update(delta_time);

        // Update camera buffer with new camera matrix
        self.system.queue.write_buffer(
            &self.camera.buffer().as_ref().unwrap().buffer,
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
            if sprite.transform_mut().changed() {
                sprite.transform_mut().change_handled();
                instance_data.push(sprite.get_instance());
            } else if change_start != i {
                self.system.queue.write_buffer(
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
            self.system.queue.write_buffer(
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
        self.system.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.system.is_surface_configured {
            return Ok(());
        }
        
        // Get the next texture to be presented by the swapchain
        let output = self.system.surface.get_current_texture()?;

        // Create a TextureView type from the Texture type which specifies how the 
        // data in the Texture should be read and used by the GPU (e.g. format, dimension, mipmap levels)
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create a CommandEncoder which is used to create a CommandBuffer which is then added to the queue
        let mut encoder = self.system.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
            // Use the bind group created earlier for the camera
            render_pass.set_bind_group(1, &self.camera.buffer().as_ref().unwrap().bind_group, &[]);
            // Create a vertex buffer at slot 0 using all (..) of self.vertex_buffer
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            // Create an index buffer using all (..) of self.index_buffer
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            
            for bind_group in &self.texture_bind_groups {
                // Set the bind group to the next texture bind group
                render_pass.set_bind_group(0, &bind_group.0, &[]);
                // Draw the instances with that texture
                render_pass.draw_indexed(0..self.num_indices, 0, bind_group.1.0 as u32..bind_group.1.1 as u32);
            }
        }

        // Add the CommandBuffer created by the CommandEncoder onto the queue
        // (Submit will accept anything that implements IntoIter)
        self.system.queue.submit(std::iter::once(encoder.finish()));
        // Display the texture
        output.present();

        Ok(())
    }
}