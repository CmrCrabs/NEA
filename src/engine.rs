use super::{FORMAT, WG_SIZE};
use crate::{cast_slice, Result};
use crate::{
    renderer::Renderer,
    scene::{Mesh, Scene},
    simulation::Simulation,
    ui::UI,
};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};

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

        let mut required_limits = wgpu::Limits::default();
        required_limits.max_storage_textures_per_shader_stage = 6;
        required_limits.max_bind_groups = 6;
        required_limits.max_push_constant_size = 8;
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

        let scene = Scene::new(&device, &window);
        let simulation = Simulation::new(&device, &queue, &shader, &scene);
        let renderer = Renderer::new(&device, &queue, &shader, &window, &simulation, &scene);
        let ui = UI::new(&device, &queue, &window, &shader, &renderer, &scene);

        Self {
            config,
            device,
            queue,
            surface,
            window: &window,
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
                    self.scene.update_camera(&event, &self.window);
                }
                match event {
                    WindowEvent::RedrawRequested => {
                        self.scene.update_redraw(&self.window);

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

                            self.simulation.initial_spectra_pass.compute(
                                &mut encoder,
                                "Initial Spectra",
                                &[
                                    &self.scene.consts_bind_group,
                                    &self.simulation.simdata.bind_group,
                                    &self.simulation.cascade.stg_bind_group,
                                ],
                                workgroup_size,
                                workgroup_size,
                            );

                            self.simulation.conjugates_pass.compute(
                                &mut encoder,
                                "Pack Conjugates",
                                &[
                                    &self.scene.consts_bind_group,
                                    &self.simulation.cascade.stg_bind_group,
                                ],
                                workgroup_size,
                                workgroup_size,
                            );
                            self.scene.mesh = Mesh::new(&self.device, &self.scene.consts);
                        }

                        // per frame computation
                        self.simulation.evolve_spectra_pass.compute(
                            &mut encoder,
                            "Evolve Spectra",
                            &[
                                &self.scene.consts_bind_group,
                                &self.simulation.cascade.stg_bind_group,
                                &self.simulation.cascade.h_displacement.stg_bind_group,
                                &self.simulation.cascade.v_displacement.stg_bind_group,
                                &self.simulation.cascade.h_slope.stg_bind_group,
                                &self.simulation.cascade.jacobian.stg_bind_group,
                            ],
                            workgroup_size,
                            workgroup_size,
                        );
                        self.simulation.fft.ifft2d(
                            &mut encoder,
                            &mut self.scene,
                            &self.simulation.simdata,
                            &self.simulation.cascade.h_displacement,
                        );
                        self.simulation.fft.ifft2d(
                            &mut encoder,
                            &mut self.scene,
                            &self.simulation.simdata,
                            &self.simulation.cascade.v_displacement,
                        );
                        self.simulation.fft.ifft2d(
                            &mut encoder,
                            &mut self.scene,
                            &self.simulation.simdata,
                            &self.simulation.cascade.h_slope,
                        );
                        self.simulation.fft.ifft2d(
                            &mut encoder,
                            &mut self.scene,
                            &self.simulation.simdata,
                            &self.simulation.cascade.jacobian,
                        );

                        self.simulation.process_deltas_pass.compute(
                            &mut encoder,
                            "Process Deltas",
                            &[
                                &self.scene.consts_bind_group,
                                &self.simulation.cascade.h_displacement.stg_bind_group,
                                &self.simulation.cascade.v_displacement.stg_bind_group,
                                &self.simulation.cascade.h_slope.stg_bind_group,
                                &self.simulation.cascade.jacobian.stg_bind_group,
                                &self.simulation.cascade.stg_bind_group,
                            ],
                            workgroup_size,
                            workgroup_size,
                        );

                        // Render Skybox
                        self.renderer.render_skybox(&mut encoder, &surface_view, &self.scene);

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
                                &self.renderer.hdri.smp_bind_group,
                                &self.simulation.cascade.displacement_map.stg_bind_group,
                                &self.simulation.cascade.normal_map.smp_bind_group,
                                &self.simulation.cascade.foam_map.smp_bind_group,
                            ],
                            wgpu::LoadOp::Load,
                            &surface_view,
                            &self.scene.mesh,
                        );

                        // UI Pass
                        let consts_copy = self.scene.consts.clone();
                        self.ui.update_cursor(&self.window);
                        let ui_frame = self.ui.context.frame();
                        self.ui.focused = super::ui::build(ui_frame, &mut self.scene.consts);
                        self.ui.render(
                            &self.device,
                            &self.queue,
                            &mut encoder,
                            &surface_view,
                            &self.renderer.sampler_bind_group,
                            &self.scene,
                        );

                        // updating some rendering logic
                        if consts_copy != self.scene.consts {
                            self.scene.consts_changed = true;
                        } else {
                            self.scene.consts_changed = false;
                        }
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
                        self.renderer.new_depth_view(&self.device, &self.window);

                        self.scene.camera.update_fov(&self.window);
                        self.scene.consts.camera_viewproj =
                            self.scene.camera.proj * self.scene.camera.view;
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
}
