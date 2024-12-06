use std::f32::consts::PI;
use std::fs::File;
use std::io::BufReader;
use std::mem::size_of;
use std::time::Instant;
use std::{mem, slice};
// use imgui::Context;
// use imgui_winit_support::winit::event::WindowEvent;
// use imgui_winit_support::{HiDpiMode, WinitPlatform};
use glam::{Mat4, Vec2, Vec3};
use image::GenericImageView;
use log::LevelFilter;
use obj::{Obj, TexturedVertex};
use shared::SceneConstants;
use wgpu::{util::DeviceExt, BindGroupLayoutDescriptor, BindGroupLayoutEntry};
use winit::keyboard::KeyCode;
use winit::{event::*, event_loop::EventLoop, window::WindowBuilder};

mod renderer;
mod renderpass;
mod ui;

type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;
const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

#[repr(C, align(16))]
struct Vertex {
    position: Vec3,
    normal: Vec3,
}

struct Camera {
    eye: Vec3,
    target: Vec3,
    up: Vec3,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

fn main() -> Result {
    // WINDOW
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new().with_title("nea").build(&event_loop)?;

    //let mut imgui = Context::create();
    //let mut platform = WinitPlatform::new(&mut imgui);
    //platform.attach_window(imgui.to_mut(), &window, HiDpiMode::Default);

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let surface = unsafe { instance.create_surface(&window)? };
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

    // TIME
    let mut scene_consts = SceneConstants {
        time: 0.0,
        frametime: 0.0,
        width: 0.0,
        height: 0.0,
        camera_proj: Mat4::ZERO,
    };
    let start_time = Instant::now();

    // CAMERA SETUP
    let mut pitch: f32 = 0.0;
    let mut yaw: f32 = 0.0;
    let mut zoom: f32 = 5.0;

    let mut camera = Camera {
        eye: Vec3::new(
            zoom * yaw.cos() * pitch.sin(),
            zoom * yaw.sin(),
            zoom * yaw.cos() * pitch.cos(),
        ),
        target: Vec3::new(0.0, 0.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        aspect: window.inner_size().width as f32 / window.inner_size().height as f32,
        fovy: 45.0,
        znear: 0.1,
        zfar: 100.0,
    };

    let mut view = Mat4::look_at_rh(camera.eye, camera.target, camera.up);
    let proj = Mat4::perspective_infinite_rh(camera.fovy.to_radians(), camera.aspect, camera.znear);
    scene_consts.camera_proj = proj * view;

    // scene const buf
    let scene_buf = device.create_buffer(&wgpu::BufferDescriptor {
        size: size_of::<SceneConstants>() as u64,
        mapped_at_creation: false,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        label: None,
    });
    let scene_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: None,
    });
    let scene_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &scene_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: scene_buf.as_entire_binding(),
        }],
        label: None,
    });
    //OBJ
    let input = BufReader::new(File::open("assets/sphere.obj")?);
    let obj: Obj<TexturedVertex, u32> = obj::load_obj(input)?;
    let vertices = &obj
        .vertices
        .iter()
        .map(|v| Vertex {
            position: v.position.into(),
            normal: v.normal.into(),
        })
        .collect::<Vec<_>>();

    let vtx_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        contents: cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
        label: None,
    });

    let idx_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        contents: cast_slice(&obj.indices),
        usage: wgpu::BufferUsages::INDEX,
        label: None,
    });

    // DEPTH BUFFER
    const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
    let size = wgpu::Extent3d {
        width: window.inner_size().width,
        height: window.inner_size().height,
        depth_or_array_layers: 1,
    };

    let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: DEPTH_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let mut depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

    // SHADERS
    let shader = device.create_shader_module(wgpu::include_spirv!(env!("shaders.spv")));

    // PIPELINE
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[
            &scene_bind_group_layout
        ],
        push_constant_ranges: &[],
        label: None,
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "main_vs",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: mem::size_of::<Vertex>() as _,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
            }],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "main_fs",
            targets: &[Some(wgpu::ColorTargetState {
                format: FORMAT,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: Some(wgpu::DepthStencilState {
            format: DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        label: None,
    });

    let mut cursor_down: bool = false;
    let mut last_frame = Instant::now();
    // EVENT LOOP
    event_loop.run(move |event, elwt| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => elwt.exit(),

            WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
                winit::keyboard::PhysicalKey::Code(KeyCode::Escape) => elwt.exit(),
                _ => {}
            },

            WindowEvent::MouseInput { state, button, .. } => match button {
                MouseButton::Left => cursor_down = state.is_pressed(),
                _ => (),
            },

            WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    MouseScrollDelta::LineDelta(_, y) => {
                        zoom -= y;
                    }
                    MouseScrollDelta::PixelDelta(winit::dpi::PhysicalPosition { y, .. }) => {
                        zoom -= y as f32;
                    }
                }
                camera.eye = Vec3::new(
                    zoom * yaw.cos() * pitch.sin(),
                    zoom * yaw.sin(),
                    zoom * yaw.cos() * pitch.cos(),
                );
                view = Mat4::look_at_rh(camera.eye, camera.target, camera.up);
                scene_consts.camera_proj = proj * view;
                queue.write_buffer(&scene_buf, 0, cast_slice(&[scene_consts]));
            }

            WindowEvent::CursorMoved { position, .. } => {
                if cursor_down {
                    match position {
                        PhysicalPosition { x, y } => {
                            yaw = (PI / window.inner_size().height as f32)
                                * (y as f32 - (window.inner_size().height as f32 / 2.0) + 0.01);
                            pitch = ((2.0 * PI) / window.inner_size().width as f32)
                                * (x as f32 - (window.inner_size().width as f32 / 2.0));
                        }
                    }
                    camera.eye = Vec3::new(
                        zoom * -yaw.cos() * pitch.sin(),
                        zoom * yaw.sin(),
                        zoom * yaw.cos() * pitch.cos(),
                    );
                    view = Mat4::look_at_rh(camera.eye, camera.target, camera.up);
                    scene_consts.camera_proj = proj * view;
                    queue.write_buffer(&scene_buf, 0, cast_slice(&[scene_consts]));
                }
            }

            WindowEvent::RedrawRequested => {
                let duration = start_time.elapsed().as_secs_f32();
                scene_consts.frametime = duration - scene_consts.time;
                scene_consts.time = duration;

                let dimensions = window.inner_size();
                scene_consts.width = dimensions.width as f32;
                scene_consts.height = dimensions.height as f32;

                queue.write_buffer(&scene_buf, 0, cast_slice(&[scene_consts]));

                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                let surface = surface.get_current_texture().unwrap();
                let surface_view = surface
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &surface_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &depth_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    label: None,
                });

                render_pass.set_pipeline(&pipeline);
                render_pass.set_vertex_buffer(0, vtx_buf.slice(..));
                render_pass.set_bind_group(0, &scene_bind_group, &[]);
                render_pass.set_index_buffer(idx_buf.slice(..), wgpu::IndexFormat::Uint32);

                render_pass.draw_indexed(0..(obj.indices.len() as _), 0, 0..1);
                drop(render_pass);

                queue.submit([encoder.finish()]);
                surface.present();
            }

            WindowEvent::Resized(size) => {
                let config = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: FORMAT,
                    width: size.width,
                    height: size.height,
                    present_mode: wgpu::PresentMode::Fifo,
                    alpha_mode: wgpu::CompositeAlphaMode::Opaque,
                    view_formats: vec![],
                };
                surface.configure(&device, &config);

                //FIXING FOV
                camera.aspect = window.inner_size().width as f32 / window.inner_size().height as f32;
                let proj = Mat4::perspective_rh(
                    camera.fovy.to_radians(),
                    camera.aspect,
                    camera.znear,
                    camera.zfar,
                );
                scene_consts.camera_proj = proj * view;
                
                queue.write_buffer(&scene_buf, 0, cast_slice(&[scene_consts]));

                // FIXING DEPTH BUFFER
                let size = wgpu::Extent3d {
                    width: window.inner_size().width,
                    height: window.inner_size().height,
                    depth_or_array_layers: 1,
                };

                let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
                    label: None,
                    size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: DEPTH_FORMAT,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                });
                depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
            }
            _ => {}
        },
        Event::AboutToWait => window.request_redraw(),
        _ => {}
    })?;
    Ok(())
}

fn cast_slice<T>(fake: &[T]) -> &[u8] {
    unsafe { slice::from_raw_parts(fake.as_ptr() as _, mem::size_of_val(fake)) }
}
