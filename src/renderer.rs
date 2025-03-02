use crate::{
    cast_slice, scene::{Mesh, Scene}, sim::{self, compute::ComputePass, fft::FourierTransform, Cascade}, standardpass::StandardPipeline, ui::{build, UI}, util::Texture, Result
};
use std::time::Instant;
use winit::keyboard::KeyCode;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::PhysicalKey,
};

pub const WG_SIZE: u32 = 8;

pub struct Renderer<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub window: &'a winit::window::Window,
    pub shader: wgpu::ShaderModule,
    pub sampler_bind_group: wgpu::BindGroup,
    pub sampler_layout: wgpu::BindGroupLayout,
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

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());
        let sampler_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                count: None,
            }],
            label: None,
        });
        let sampler_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &sampler_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&sampler),
            }],
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
            sampler_layout,
            sampler_bind_group,
            depth_view,
        }
    }

    pub fn run(
        &mut self,
        event_loop: EventLoop<()>,
        mut scene: Scene,
        mut ui: UI,
        cascade: Cascade,
    ) -> Result {
        let mut last_frame = Instant::now();
        let simdata = sim::SimData::new(&self, &scene.consts);
        let hdri = Texture::from_file(&self, "HDRI", "./assets/hdris/kloofendal.exr");
        let standard_pass = StandardPipeline::new(&self, &scene, &cascade, &hdri);
        let workgroup_size = scene.consts.sim.size / WG_SIZE;

        let initial_spectra_pass = ComputePass::new(
            &[&scene.consts_layout, &simdata.layout, &cascade.stg_layout],
            &self,
            "Initial Spectra",
            "initial_spectra::main",
        );
        let butterfly_precompute_pass = ComputePass::new(
            &[&scene.consts_layout, &simdata.layout],
            &self,
            "Precompute Butterfly",
            "fft::precompute_butterfly",
        );
        let conjugates_pass = ComputePass::new(
            &[&scene.consts_layout, &cascade.stg_layout],
            &self,
            "Pack Conjugates",
            "initial_spectra::pack_conjugates",
        );
        let evolve_spectra_pass = ComputePass::new(
            &[
                &scene.consts_layout,
                &cascade.stg_layout,
                &cascade.h_displacement.stg_layout,
                &cascade.v_displacement.stg_layout,
                &cascade.h_slope.stg_layout,
                &cascade.jacobian.stg_layout,
            ],
            &self,
            "Evolve Spectra",
            "evolve_spectra::main",
        );
        let process_deltas_pass = ComputePass::new(
            &[
                &scene.consts_layout,
                &cascade.h_displacement.stg_layout,
                &cascade.v_displacement.stg_layout,
                &cascade.h_slope.stg_layout,
                &cascade.jacobian.stg_layout,
                &cascade.stg_layout,
            ],
            &self,
            "Process Deltas",
            "process_deltas::main",
        );
        let fft = FourierTransform::new(&scene, &simdata, &self);

        simdata
            .gaussian_tex
            .write(&self.queue, cast_slice(&simdata.gaussian_noise.clone()), 16);

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
                        scene.update_redraw(&self.window);

                        let surface = self.surface.get_current_texture().unwrap();
                        let surface_view = surface
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());
                        let mut encoder = self
                            .device
                            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                        // TODO: move out of frame by frame
                        butterfly_precompute_pass.compute(
                            &mut encoder,
                            "Precompute Butterfly",
                            &[&scene.consts_bind_group, &simdata.bind_group],
                            scene.consts.sim.size.ilog2(),
                            scene.consts.sim.size / WG_SIZE,
                        );

                        // Compute Initial spectrum on param change
                        // TODO: change to consts changed
                        if scene.consts_changed {
                            scene.write(&self.queue);

                            initial_spectra_pass.compute(
                                &mut encoder,
                                "Initial Spectra",
                                &[
                                    &scene.consts_bind_group,
                                    &simdata.bind_group,
                                    &cascade.stg_bind_group,
                                ],
                                workgroup_size,
                                workgroup_size,
                            );

                            conjugates_pass.compute(
                                &mut encoder,
                                "Pack Conjugates",
                                &[&scene.consts_bind_group, &cascade.stg_bind_group],
                                workgroup_size,
                                workgroup_size,
                            );

                            scene.mesh = Mesh::new(&self.device, &scene.consts);
                        }

                        // per frame computation
                        evolve_spectra_pass.compute(
                            &mut encoder, "Evolve Spectra", &[ 
                                &scene.consts_bind_group, 
                                &cascade.stg_bind_group, 
                                &cascade.h_displacement.stg_bind_group, 
                                &cascade.v_displacement.stg_bind_group,
                                &cascade.h_slope.stg_bind_group,
                                &cascade.jacobian.stg_bind_group,
                            ],
                            workgroup_size,
                            workgroup_size,
                        );

                        fft.ifft2d(&mut encoder, &mut scene, &simdata, &cascade.h_displacement);
                        fft.ifft2d(&mut encoder, &mut scene, &simdata, &cascade.v_displacement);
                        fft.ifft2d(&mut encoder, &mut scene, &simdata, &cascade.h_slope);
                        fft.ifft2d(&mut encoder, &mut scene, &simdata, &cascade.jacobian);

                        process_deltas_pass.compute(
                            &mut encoder,
                            "Process Deltas",
                            &[
                                &scene.consts_bind_group,
                                &cascade.h_displacement.stg_bind_group,
                                &cascade.v_displacement.stg_bind_group,
                                &cascade.h_slope.stg_bind_group,
                                &cascade.jacobian.stg_bind_group,
                                &cascade.stg_bind_group,
                            ],
                            workgroup_size,
                            workgroup_size,
                        );

                        // Standard Pass
                        self.queue
                            .write_buffer(&scene.consts_buf, 0, cast_slice(&[scene.consts]));
                        let mut pass =
                            standard_pass.render(&mut encoder, &surface_view, &self.depth_view);
                        pass.set_pipeline(&standard_pass.pipeline);
                        pass.set_bind_group(0, &scene.consts_bind_group, &[]);
                        pass.set_bind_group(1, &self.sampler_bind_group, &[]);
                        pass.set_bind_group(2, &hdri.smp_bind_group, &[]);
                        pass.set_bind_group(3, &cascade.displacement_map.stg_bind_group, &[]);
                        pass.set_bind_group(4, &cascade.normal_map.stg_bind_group, &[]);
                        pass.set_bind_group(5, &cascade.foam_map.stg_bind_group, &[]);
                        pass.set_vertex_buffer(0, scene.mesh.vtx_buf.slice(..));
                        pass.set_index_buffer(
                            scene.mesh.idx_buf.slice(..),
                            wgpu::IndexFormat::Uint32,
                        );
                        pass.draw_indexed(0..(scene.mesh.length as _), 0, 0..1);
                        drop(pass);

                        // UI Pass
                        let consts_copy = scene.consts.clone();
                        ui.update_cursor(&self.window);
                        let ui_frame = ui.context.frame();
                        ui.focused = build(ui_frame, &mut scene.consts);
                        ui.render(
                            &self.device,
                            &self.queue,
                            &mut encoder,
                            &surface_view,
                            &self.sampler_bind_group,
                            &scene,
                        );
                        if consts_copy != scene.consts {
                            scene.consts_changed = true;
                        } else {
                            scene.consts_changed = false;
                        }

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
