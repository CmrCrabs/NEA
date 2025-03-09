use super::{FORMAT, WG_SIZE};
use crate::{cast_slice, Result};
use {
    renderer::Renderer,
    scene::{Mesh, Scene},
    crate::sim::Simulation,
    ui::UI,
};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};

pub mod renderer;
pub mod scene;
pub mod ui;
pub mod util;

pub struct Engine<'a> {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub window: &'a winit::window::Window,
    pub surface: wgpu::Surface<'a>,
    pub simulation: Simulation,
    pub renderer: Renderer,
    pub scene: Scene,
    pub ui: UI,
}

impl<'a> Engine<'a> {
    pub fn new(window: &'a winit::window::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window).unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .expect("failed to create adapter");

        let required_limits = wgpu::Limits {
            max_storage_textures_per_shader_stage: 6,
            max_bind_groups: 6,
            max_push_constant_size: 8,
            ..Default::default()
        };
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES
                    | wgpu::Features::VERTEX_WRITABLE_STORAGE
                    | wgpu::Features::PUSH_CONSTANTS,
                required_limits,
                memory_hints: wgpu::MemoryHints::Performance,
                label: None,
            },
            None,
        ))
        .expect("failed to create device & queue");

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

        let scene = Scene::new(&device, window);
        let simulation = Simulation::new(&device, &queue, &shader, &scene);
        let renderer = Renderer::new(&device, &queue, &shader, window, &simulation, &scene);
        let ui = UI::new(&device, &queue, window, &shader, &renderer, &scene);

        Self {
            config,
            device,
            queue,
            surface,
            window,
            simulation,
            scene,
            renderer,
            ui,
        }
    }

    pub fn run(&mut self, event_loop: EventLoop<()>) -> Result {
        let mut last_frame = std::time::Instant::now();
        let mut first_frame = true;
        let workgroup_size = self.scene.consts.sim.size / WG_SIZE;

        event_loop.run(move |event, elwt| match event {
            Event::AboutToWait => self.window.request_redraw(),
            Event::NewEvents(_) => {
                let now = std::time::Instant::now();
                self.ui.context.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::WindowEvent { event, .. } => {
                self.ui.handle_events(&event);
                if !self.ui.focused {
                    self.scene.update_camera(&event, self.window);
                }
                match event {
                    WindowEvent::RedrawRequested => {
                        self.scene.update_redraw(self.window);

                        let surface = self
                            .surface
                            .get_current_texture()
                            .expect("failed to get surface");
                        let surface_view = surface
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());
                        let mut encoder = self
                            .device
                            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                        if first_frame {
                            self.simulation.butterfly_precompute_pass.compute(
                                &mut encoder,
                                "Precompute Butterfly",
                                &[
                                    &self.scene.consts_bind_group,
                                    &self.simulation.simdata.bind_group,
                                ],
                                self.scene.consts.sim.size.ilog2(),
                                self.scene.consts.sim.size / WG_SIZE,
                            );
                        }

                        // Compute Initial spectrum on param change
                        if self.scene.consts_changed {
                            self.scene.write(&self.queue);

                            self.simulation.compute_initial(
                                &mut encoder, 
                                &[
                                    &self.scene.consts_bind_group,
                                    &self.simulation.simdata.bind_group,
                                    &self.simulation.cascade0.bind_group,
                                ],
                                0, 
                                workgroup_size,
                                workgroup_size,
                            );
                            self.simulation.compute_initial(
                                &mut encoder, 
                                &[
                                    &self.scene.consts_bind_group,
                                    &self.simulation.simdata.bind_group,
                                    &self.simulation.cascade1.bind_group,
                                ],
                                1, 
                                workgroup_size,
                                workgroup_size,
                            );
                            self.simulation.compute_initial(
                                &mut encoder, 
                                &[
                                    &self.scene.consts_bind_group,
                                    &self.simulation.simdata.bind_group,
                                    &self.simulation.cascade2.bind_group,
                                ],
                                2, 
                                workgroup_size,
                                workgroup_size,
                            );

                            self.scene.mesh = Mesh::new(&self.device, &self.scene.consts);
                        }

                        // per frame computation
                        self.simulation.compute_cascade(&mut encoder, &self.simulation.cascade0, &mut self.scene, workgroup_size);
                        self.simulation.compute_cascade(&mut encoder, &self.simulation.cascade1, &mut self.scene, workgroup_size);
                        self.simulation.compute_cascade(&mut encoder, &self.simulation.cascade2, &mut self.scene, workgroup_size);

                        // Render Skybox
                        self.renderer
                            .render_skybox(&mut encoder, &surface_view, &self.scene);

                        // Standard Render Pass
                        self.queue.write_buffer(
                            &self.scene.consts_buf,
                            0,
                            cast_slice(&[self.scene.consts]),
                        );
                        self.renderer.render_standard(
                            &mut encoder,
                            &self.renderer.std_pipeline,
                            &[
                                &self.scene.consts_bind_group,
                                &self.renderer.sampler_bind_group,
                                &self.renderer.hdri.bind_group,
                                &self.simulation.cascade0.bind_group,
                                &self.simulation.cascade1.bind_group,
                                &self.simulation.cascade2.bind_group,
                            ],
                            &surface_view,
                            &self.scene.mesh,
                            self.scene.consts.sim.instances,
                        );

                        // UI Pass
                        let consts_copy = self.scene.consts;
                        self.ui.update_cursor(self.window);
                        let ui_frame = self.ui.context.frame();
                        self.ui.focused = ui::build(ui_frame, &mut self.scene.consts);
                        self.ui.render(
                            &self.device,
                            &self.queue,
                            &mut encoder,
                            &surface_view,
                            &self.renderer.sampler_bind_group,
                            &self.scene,
                        );

                        // updating some rendering logic
                        self.scene.consts_changed = consts_copy != self.scene.consts;
                        first_frame = false;

                        // Submitting queue to be computed
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
                        self.renderer.new_depth_view(&self.device, self.window);

                        self.scene.camera.update_fov(self.window);
                        self.scene.consts.camera_viewproj =
                            self.scene.camera.proj * self.scene.camera.view;
                    }
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::KeyboardInput { event, .. } => if let PhysicalKey::Code(KeyCode::Escape) = event.physical_key {
                        elwt.exit()
                    },
                    _ => {}
                }
            }
            _ => {}
        })?;
        Ok(())
    }
}
