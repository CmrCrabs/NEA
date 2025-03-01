use crate::cast_slice;
use glam::{Mat4, Vec3, Vec4};
use shared::{Constants, ShaderConstants, SimConstants};
use std::{f32::consts::PI, mem, time::Instant};
use wgpu::{util::DeviceExt, Buffer};
use winit::event::WindowEvent;
use winit::{dpi::PhysicalPosition, event::MouseScrollDelta, window::Window};

pub struct Scene {
    start_time: Instant,
    cursor_down: bool,
    pub camera: Camera,
    pub mesh: Mesh,
    pub consts: Constants,
    pub consts_layout: wgpu::BindGroupLayout,
    pub consts_buf: wgpu::Buffer,
    pub consts_bind_group: wgpu::BindGroup,
    pub consts_changed: bool,
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
    pub _vertices: Vec<OceanVertex>,
    pub idx_buf: Buffer,
    pub vtx_buf: Buffer,
    pub length: usize,
}

//TODO: maybe remove if normal sampled directly from texture
#[repr(C, align(16))]
pub struct OceanVertex {
    pos: Vec4,
    uv: glam::UVec2,
    normal: Vec4,
}

impl Scene {
    pub fn new(window: &Window, device: &wgpu::Device) -> Self {
        let cursor_down = false;
        let camera = Camera::new(window);
        let consts = Constants {
            time: 0.0,
            deltatime: 0.0,
            width: 0.0,
            height: 0.0,
            camera_proj: camera.proj * camera.view,
            eye: camera.eye.extend(1.0),
            shader: ShaderConstants::default(),
            sim: SimConstants::default(),
        };
        let mesh = Mesh::new(device, &consts);
        let start_time = Instant::now();

        let consts_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT | wgpu::ShaderStages::COMPUTE,
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
        let consts_buf = device.create_buffer(&wgpu::BufferDescriptor {
            size: mem_size as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            label: Some("Consts Buffer"),
        });
        let consts_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &consts_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: consts_buf.as_entire_binding(),
            }],
            label: Some("Consts Bind Group"),
        });

        let consts_changed = true;

        Self {
            cursor_down,
            start_time,
            consts,
            camera,
            mesh,
            consts_layout,
            consts_buf,
            consts_bind_group,
            consts_changed,
        }
    }

    pub fn update_redraw(&mut self, window: &Window) {
        // Update (consts)
        let duration = self.start_time.elapsed().as_secs_f32();
        self.consts.deltatime = duration - self.consts.time;
        self.consts.time = duration;

        self.consts.eye = self.camera.eye.extend(1.0);

        let w = self.consts.sim.size as f32 * self.consts.sim.mesh_step * self.consts.shader.distance_factor;
        self.consts.shader.light = Vec4::new(self.consts.shader.sun_distance, self.consts.shader.sun_height, self.consts.shader.sun_distance, 1.0 / w) * w;
        self.consts.shader.light =
            Mat4::from_rotation_x(self.consts.shader.sun_angle) * self.consts.shader.light;
        self.consts.sim.logsize = self.consts.sim.size.ilog2();

        // "Redraw"
        let dimensions = window.inner_size();
        self.consts.width = dimensions.width as f32;
        self.consts.height = dimensions.height as f32;
    }

    pub fn write(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.consts_buf, 0, cast_slice(&[self.consts]));
    }

    pub fn update_camera(&mut self, event: &WindowEvent, window: &Window) {
        match event {
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
    pub fn new(device: &wgpu::Device, consts: &Constants) -> Self {
        let scale = consts.sim.size;
        let step = consts.sim.mesh_step;
        let mut vertices: Vec<OceanVertex> = vec![];
        for z in 0..scale {
            for x in 0..scale {
                let pos = Vec4::new(x as f32 * step, 0.0, z as f32 * step, 1.0);
                let uv = glam::UVec2::new(x, z);
                vertices.push(OceanVertex {
                    pos,
                    uv,
                    normal: Vec4::ZERO,
                });
            }
        }
        let vtx_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents: cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
            label: None,
        });

        let mut indices: Vec<u32> = vec![];
        for y in 0..scale - 1 {
            for x in 0..scale - 1 {
                indices.push(x + y * scale);
                indices.push((x + 1) + (y + 1) * scale);
                indices.push(x + (y + 1) * scale);
                indices.push(x + y * scale);
                indices.push((x + 1) + y * scale);
                indices.push((x + 1) + (y + 1) * scale);
            }
        }

        let idx_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents: cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
            label: None,
        });
        let length = indices.len();

        Self {
            _vertices: vertices,
            vtx_buf,
            idx_buf,
            length,
        }
    }
}

impl Camera {
    pub fn new(window: &Window) -> Camera {
        let pitch: f32 = 0.0;
        let yaw: f32 = -10.0;
        let zoom: f32 = 20.0;

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
