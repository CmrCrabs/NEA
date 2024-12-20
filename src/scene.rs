use std::{f32::consts::PI, time::Instant};
use crate::cast_slice;
use std::io::BufReader;
use std::fs::File;
use obj::Obj;
use glam::{Vec3, Mat4};
use wgpu::{Buffer, util::DeviceExt};
use winit::{dpi::PhysicalPosition, event::MouseScrollDelta, window::Window};
use shared::SceneConstants;

pub struct Scene {
    start_time: Instant,
    pub consts: SceneConstants,
    pub camera: Camera,
    pub mesh: Mesh,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub idx_buf: Buffer,
    pub vtx_buf: Buffer,
    pub length: usize,
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

#[repr(C, align(16))]
pub struct Vertex {
    position: Vec3,
    normal: Vec3,
}

impl Scene {
    pub fn new(window: &Window, device: &wgpu::Device) -> Self {
        let camera = Camera::new(window);

        let consts = SceneConstants {
            time: 0.0,
            frametime: 0.0,
            width: 0.0,
            height: 0.0,
            camera_proj: camera.proj * camera.view,
        };

        let mesh = Scene::get_mesh(device);
        let start_time = Instant::now();

        Self { 
            start_time,
            consts,
            camera,
            mesh,
        }
    }

    fn get_mesh(device: &wgpu::Device) -> Mesh {
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

        Mesh {
            vertices,
            vtx_buf,
            idx_buf,
            length,
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
        self.proj = Mat4::perspective_rh(
            self.fovy.to_radians(),
            self.aspect,
            self.znear,
            self.zfar,
        );
    }
} 
