use crate::ui::gen_interface;
use crate::{cast_slice, renderpass::StandardPipeline, scene::Scene, ui::UI, Result};
use std::time::Instant;
use wgpu::ShaderModule;
use winit::keyboard::KeyCode;
use winit::{
    event::{Event, MouseButton, WindowEvent},
    event_loop::EventLoop,
    keyboard::PhysicalKey,
};

pub struct Renderer {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub window: winit::window::Window,
    pub shader: ShaderModule,
    pub tex_layout: wgpu::BindGroupLayout,
}

pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

impl Renderer {
    pub fn new(window: winit::window::Window) -> Renderer {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(&window).unwrap() };
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::VERTEX_WRITABLE_STORAGE,
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ))
        .unwrap();
        let shader = device.create_shader_module(wgpu::include_spirv!(env!("shaders.spv")));

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: FORMAT,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
        };

        let tex_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
            label: None,
        });

        Self {
            surface,
            device,
            queue,
            window,
            config,
            shader,
            tex_layout,
        }
    }

    pub fn run(&mut self, event_loop: EventLoop<()>, mut scene: Scene, mut ui: UI) -> Result {
        let mut cursor_down: bool = false;
        let mut last_frame = Instant::now();

        let mut standard_pipeline =
            StandardPipeline::new(&self.device, &self.window, &self.shader, &scene);

        event_loop.run(move |event, elwt| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                ui.context.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::AboutToWait => self.window.request_redraw(),

            Event::WindowEvent { event, .. } => {
                ui.handle_events(&event, &self.window);
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),

                    WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
                        PhysicalKey::Code(KeyCode::Escape) => elwt.exit(),
                        _ => (),
                    },

                    WindowEvent::MouseInput { state, button, .. } => match button {
                        MouseButton::Left => cursor_down = state.is_pressed(),
                        _ => (),
                    },

                    // move to scene handle_events
                    WindowEvent::MouseWheel { delta, .. } => {
                        scene.camera.zoom(delta);
                        scene.consts.camera_proj = scene.camera.proj * scene.camera.view;
                        self.queue.write_buffer(
                            &standard_pipeline.scene_buf,
                            0,
                            cast_slice(&[scene.consts]),
                        );
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        if cursor_down {
                            scene.camera.pan(position, &self.window);
                            scene.consts.camera_proj = scene.camera.proj * scene.camera.view;
                            self.queue.write_buffer(
                                &standard_pipeline.scene_buf,
                                0,
                                cast_slice(&[scene.consts]),
                            );
                        }
                    }

                    WindowEvent::RedrawRequested => {
                        scene.redraw(&self.window);
                        self.queue.write_buffer(
                            &standard_pipeline.scene_buf,
                            0,
                            cast_slice(&[scene.consts]),
                        );

                        let surface = self.surface.get_current_texture().unwrap();
                        let surface_view = surface
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        // Standard Pass
                        let mut encoder = self
                            .device
                            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                        let mut pass = standard_pipeline.render(&mut encoder, &surface_view);

                        pass.set_pipeline(&standard_pipeline.pipeline);
                        pass.set_bind_group(0, &standard_pipeline.scene_bind_group, &[]);
                        pass.set_vertex_buffer(0, scene.mesh.vtx_buf.slice(..));
                        pass.set_index_buffer(
                            scene.mesh.idx_buf.slice(..),
                            wgpu::IndexFormat::Uint32,
                        );
                        pass.draw_indexed(0..(scene.mesh.length as _), 0, 0..1);
                        drop(pass);

                        // UI Pass
                        ui.update_cursor(&self.window);
                        let ui_frame = ui.context.frame();
                        gen_interface(ui_frame);
                        ui.render(
                            &self.device,
                            &self.queue,
                            &mut encoder,
                            &surface_view,
                            &scene,
                        );

                        self.queue.submit([encoder.finish()]);
                        surface.present();
                    }

                    WindowEvent::Resized(size) => {
                        // Update Config
                        self.config = wgpu::SurfaceConfiguration {
                            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                            format: FORMAT,
                            width: size.width,
                            height: size.height,
                            present_mode: wgpu::PresentMode::Fifo,
                            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
                            view_formats: vec![],
                        };
                        self.surface.configure(&self.device, &self.config);

                        //Update FOV
                        scene.camera.update_fov(&self.window);
                        scene.consts.camera_proj = scene.camera.proj * scene.camera.view;
                        self.queue.write_buffer(
                            &standard_pipeline.scene_buf,
                            0,
                            cast_slice(&[scene.consts]),
                        );
                        standard_pipeline.depth_view =
                            StandardPipeline::new_depth_view(&self.window, &self.device);
                    }
                    _ => {}
                }
            }
            _ => {}
        })?;
        Ok(())
    }
}