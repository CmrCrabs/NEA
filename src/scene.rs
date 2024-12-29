use crate::cast_slice;
use std::mem;
use glam::{Mat4, Vec3};
use obj::Obj;
use shared::{Constants, ShaderConstants, SimConstants};
use std::fs::File;
use std::io::BufReader;
use std::{f32::consts::PI, time::Instant};
use wgpu::BindGroupLayout;
use wgpu::{util::DeviceExt, Buffer};
use winit::event::WindowEvent;
use winit::{dpi::PhysicalPosition, event::MouseScrollDelta, window::Window};

pub struct Scene {
    start_time: Instant,
    cursor_down: bool,
    pub consts: Constants,
    pub scene_layout: BindGroupLayout,
    pub camera: Camera,
    pub mesh: Mesh,
    pub mem_size: u64,
}

pub struct Camera {
    pub proj: Mat4,
    pub view: Mat4,
    pitch: f32,
    yaw: f32,
    zoom: f32,
    eye: Vec3,
    target: Vec3,
    up: Vec3,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub idx_buf: Buffer,
    pub vtx_buf: Buffer,
    pub length: usize,
}
#[repr(C, align(16))]
pub struct Vertex {
    position: Vec3,
    normal: Vec3,
}

impl Scene {
    pub fn new(window: &Window, device: &wgpu::Device) -> Self {
        let cursor_down = false;
        let camera = Camera::new(window);
        let consts = Constants {
            time: 0.0,
            frametime: 0.0,
            width: 0.0,
            height: 0.0,
            camera_proj: camera.proj * camera.view,
            //view: camera.eye,
            shader: ShaderConstants::default(),
            sim: SimConstants::default(),
        };
        let mesh = Mesh::new(device);
        let start_time = Instant::now();

        let scene_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let mem_size = (mem::size_of::<Constants>()
            + mem::size_of::<SimConstants>()
            + mem::size_of::<ShaderConstants>()) as u64;

        Self {
            cursor_down,
            start_time,
            consts,
            camera,
            mesh,
            scene_layout,
            mem_size,
        }
    }

    pub fn redraw(&mut self, window: &Window) {
        let duration = self.start_time.elapsed().as_secs_f32();
        self.consts.frametime = duration - self.consts.time;
        self.consts.time = duration;

        let dimensions = window.inner_size();
        self.consts.width = dimensions.width as f32;
        self.consts.height = dimensions.height as f32;
    }

    pub fn handle_events(&mut self, event: &WindowEvent, window: &Window) {
        match event {
            WindowEvent::Resized(_) => {
                self.camera.update_fov(window);
                self.consts.camera_proj = self.camera.proj * self.camera.view;
            }
            WindowEvent::MouseInput { state, button, .. } => match button {
                winit::event::MouseButton::Left => self.cursor_down = state.is_pressed(),
                _ => {}
            },
            WindowEvent::MouseWheel { delta, .. } => {
                self.camera.zoom(*delta);
                self.consts.camera_proj = self.camera.proj * self.camera.view;
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.cursor_down {
                    self.camera.pan(*position, window);
                    self.consts.camera_proj = self.camera.proj * self.camera.view;
                }
            }
            _ => {}
        }
    }
}

impl Mesh {
    pub fn new(device: &wgpu::Device) -> Self {
        let input = BufReader::new(File::open("assets/sphere.obj").unwrap());
        let obj: Obj<obj::TexturedVertex, u32> = obj::load_obj(input).unwrap();
        let vertices = obj
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
        let length = obj.indices.len();

        Self {
            vertices,
            vtx_buf,
            idx_buf,
            length,
        }
    }
}

impl Camera {
    pub fn new(window: &Window) -> Camera {
        let pitch: f32 = 0.0;
        let yaw: f32 = 0.0;
        let zoom: f32 = 5.0;

        let mut camera = Camera {
            pitch,
            yaw,
            zoom,
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
            proj: Mat4::ZERO,
            view: Mat4::ZERO,
        };

        camera.proj = Mat4::perspective_rh(
            camera.fovy.to_radians(),
            camera.aspect,
            camera.znear,
            camera.zfar,
        );
        camera.view = Mat4::look_at_rh(camera.eye, camera.target, camera.up);
        camera
    }

    pub fn zoom(&mut self, delta: MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(_, y) => {
                self.zoom -= y;
            }
            MouseScrollDelta::PixelDelta(winit::dpi::PhysicalPosition { y, .. }) => {
                self.zoom -= y as f32;
            }
        }
        self.eye = Vec3::new(
            self.zoom * self.yaw.cos() * self.pitch.sin(),
            self.zoom * self.yaw.sin(),
            self.zoom * self.yaw.cos() * self.pitch.cos(),
        );
        self.view = Mat4::look_at_rh(self.eye, self.target, self.up);
    }

    pub fn pan(&mut self, position: PhysicalPosition<f64>, window: &Window) {
        match position {
            PhysicalPosition { x, y } => {
                self.yaw = (PI / window.inner_size().height as f32)
                    * (y as f32 - (window.inner_size().height as f32 / 2.0) + 0.01);
                self.pitch = ((2.0 * PI) / window.inner_size().width as f32)
                    * (x as f32 - (window.inner_size().width as f32 / 2.0));
            }
        }
        self.eye = Vec3::new(
            self.zoom * -self.yaw.cos() * self.pitch.sin(),
            self.zoom * self.yaw.sin(),
            self.zoom * self.yaw.cos() * self.pitch.cos(),
        );
        self.view = Mat4::look_at_rh(self.eye, self.target, self.up);
    }

    pub fn update_fov(&mut self, window: &Window) {
        self.aspect = window.inner_size().width as f32 / window.inner_size().height as f32;
        self.proj =
            Mat4::perspective_rh(self.fovy.to_radians(), self.aspect, self.znear, self.zfar);
    }
}
