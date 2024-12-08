use wgpu::ShaderModule;
use winit::{event_loop::EventLoop,event::{Event, WindowEvent, MouseButton}};
use winit::keyboard::KeyCode;
use crate::{cast_slice, renderpass::StandardPipeline, scene::Scene, Result};

pub struct Renderer {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub window: winit::window::Window,
    pub shader: ShaderModule,
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

        Self {
            surface,
            device,
            queue,
            window,
            config,
            shader,
        }
    }

    pub fn run(&mut self, event_loop: EventLoop<()>, mut scene: Scene) -> Result {
        let mut cursor_down: bool = false;
        let scene_buf = StandardPipeline::new_scene_buf(&self.device);
        let mut standard_pipeline = StandardPipeline::new(&self.device, &self.window, &self.shader, &scene_buf);

        event_loop.run(move |event, elwt| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => elwt.exit(),

                WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
                    winit::keyboard::PhysicalKey::Code(KeyCode::Escape) => elwt.exit(),
                    _ => (),
                },

                WindowEvent::MouseInput { state, button, .. } => match button {
                    MouseButton::Left => cursor_down = state.is_pressed(),
                    _ => (),
                },

                WindowEvent::MouseWheel { delta, .. } => {
                    scene.camera.zoom(delta);
                    scene.consts.camera_proj = scene.camera.proj * scene.camera.view;
                    //todo
                    self.queue.write_buffer(&scene_buf, 0, cast_slice(&[scene.consts]));
                }

                WindowEvent::CursorMoved { position, .. } => if cursor_down {
                    scene.camera.pan(position, &self.window);
                    scene.consts.camera_proj = scene.camera.proj * scene.camera.view;
                    self.queue.write_buffer(&scene_buf, 0, cast_slice(&[scene.consts]));
                }

                WindowEvent::RedrawRequested => {
                    scene.redraw(&self.window);
                    self.queue.write_buffer(&scene_buf, 0, cast_slice(&[scene.consts]));

                    let surface = self.surface.get_current_texture().unwrap();
                    let surface_view = surface.texture.create_view(&wgpu::TextureViewDescriptor::default());

                    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                    let mut standard_pass = standard_pipeline.render(&mut encoder, &surface_view);

                    standard_pass.set_pipeline(&standard_pipeline.pipeline);
                    standard_pass.set_bind_group(0, &standard_pipeline.scene_bind_group, &[]);
                    standard_pass.set_vertex_buffer(0, scene.mesh.vtx_buf.slice(..));
                    standard_pass.set_index_buffer(scene.mesh.idx_buf.slice(..), wgpu::IndexFormat::Uint32);
                    standard_pass.draw_indexed(0..(scene.mesh.length as _), 0, 0..1);
                    drop(standard_pass);

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

                    //Fix FOV
                    scene.camera.update_fov(&self.window);
                    scene.consts.camera_proj = scene.camera.proj * scene.camera.view;
                    self.queue.write_buffer(&scene_buf, 0, cast_slice(&[scene.consts]));
                    // if broken change from self to size
                    standard_pipeline.depth_view = StandardPipeline::new_depth_view(&self.window, &self.device);
                }
                _ => {}
            },
            Event::AboutToWait => self.window.request_redraw(),
            _ => {}
        })?;
        Ok(())
    }
}
