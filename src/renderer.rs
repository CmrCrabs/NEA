use crate::{
    cast_slice,
    scene::Scene,
    standardpass::StandardPipeline,
    ui::{build, UI},
    Result,
};
use std::time::Instant;
use winit::keyboard::KeyCode;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::PhysicalKey,
};

pub struct Renderer<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub window: &'a winit::window::Window,
    pub shader: wgpu::ShaderModule,
    pub tex_layout: wgpu::BindGroupLayout,
    pub depth_view: wgpu::TextureView,
}

pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

impl<'a> Renderer<'a> {
    pub fn new(window: &'a winit::window::Window) -> Renderer<'a> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window).unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::VERTEX_WRITABLE_STORAGE | wgpu::Features::TEXTURE_FORMAT_16BIT_NORM,
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
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
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

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

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: window.inner_size().width,
                height: window.inner_size().height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            surface,
            device,
            queue,
            window: &window,
            config,
            shader,
            tex_layout,
            depth_view,
        }
    }

    pub fn run(&mut self, event_loop: EventLoop<()>, mut scene: Scene, mut ui: UI) -> Result {
        let mut last_frame = Instant::now();
        let standard_pipeline = StandardPipeline::new(&self.device, &self.shader, &scene);
        event_loop.run(move |event, elwt| match event {
            Event::AboutToWait => self.window.request_redraw(),
            Event::NewEvents(_) => {
                let now = Instant::now();
                ui.context.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::WindowEvent { event, .. } => {
                ui.handle_events(&event);
                if !ui.focused {
                    scene.update_camera(&event, &self.window);
                }
                match event {
                    WindowEvent::RedrawRequested => {
                        scene.redraw(&self.window);
                        let surface = self.surface.get_current_texture().unwrap();
                        let surface_view = surface
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());
                        let mut encoder = self
                            .device
                            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                        // Standard Pass
                        self.queue.write_buffer(
                            &standard_pipeline.scene_buf,
                            0,
                            cast_slice(&[scene.consts]),
                        );
                        let mut pass =
                            standard_pipeline.render(&mut encoder, &surface_view, &self.depth_view);
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
                        ui.focused = build(ui_frame, &mut scene.consts);
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
                            present_mode: wgpu::PresentMode::AutoNoVsync,
                            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
                            view_formats: vec![],
                            desired_maximum_frame_latency: 2,
                        };
                        self.surface.configure(&self.device, &self.config);
                        self.new_depth_view();

                        scene.camera.update_fov(&self.window);
                        scene.consts.camera_proj = scene.camera.proj * scene.camera.view;
                    }
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
                        PhysicalKey::Code(KeyCode::Escape) => elwt.exit(),
                        _ => {}
                    },
                    _ => {}
                }
            }
            _ => {}
        })?;
        Ok(())
    }

    pub fn new_depth_view(&mut self) {
        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: self.window.inner_size().width,
                height: self.window.inner_size().height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        self.depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }
}
