use glam::{Vec2, Vec4};
use crate::engine::util::{bind_group_descriptor, Texture};
use shared::Constants;

pub struct SimData {
    pub gaussian_noise: Vec<Vec4>,
    pub gaussian_tex: Texture,
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl SimData {
    pub fn new(device: &wgpu::Device, consts: &Constants) -> Self {
        let gaussian_tex = Texture::new_storage(
            consts.sim.size,
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            device,
            "Gaussian",
        );
        let gaussian_noise = Self::guassian_noise(consts);

        let butterfly_tex = Texture::new_storage(
            consts.sim.size.ilog2(),
            consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            device,
            "Butterfly",
        );

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                bind_group_descriptor(0, wgpu::TextureFormat::Rgba32Float),
                bind_group_descriptor(1, wgpu::TextureFormat::Rgba32Float),
            ],
            label: Some("Sim Data Layout"),
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&gaussian_tex.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&butterfly_tex.view),
                },
            ],
            label: Some("Sim Data Textures"),
        });

        Self {
            gaussian_tex,
            gaussian_noise,
            bind_group,
            layout,
        }
    }

    fn guassian_noise(consts: &Constants) -> Vec<Vec4> {
        let mut rng = Xoshiro256plus::new(consts.sim.seed as _);
        let mut data = vec![];
        for _ in 0..(consts.sim.size * consts.sim.size) {
            let gaussian_pair =
                Self::gaussian_number(
                    rng.next() as _,
                    rng.next() as _,
            );
            data.push(Vec4::new(gaussian_pair.x, gaussian_pair.y, 0.0, 1.0));
        }
        data
    }
    // box muller transform, gaussian pair technically not needed but is slightly cooler
    fn gaussian_number(u1: f32, u2: f32) -> Vec2 {
        Vec2::new(
            (-2.0 * u1.ln()).sqrt() * (2.0 * std::f32::consts::PI * u2).cos(),
            (-2.0 * u1.ln()).sqrt() * (2.0 * std::f32::consts::PI * u2).sin(),
        )
    }
}

struct Xoshiro256plus {
    seed: [u64; 4]
}

impl Xoshiro256plus {
    pub fn rol64(x: u64, k: i32) -> u64 {
        (x << k) | (x >> (64 - k))
    }
    fn new(seed: u64) -> Self {
        let mut rng = SplitMix::new(seed);
        Xoshiro256plus {
            seed: [rng.next(), rng.next(), rng.next(), rng.next()],
        }
    }
    fn next(&mut self) -> f64 {
        let result = self.seed[0].wrapping_add(self.seed[3]);
        let t = self.seed[1] << 17;

        self.seed[2] ^= self.seed[0];
        self.seed[3] ^= self.seed[1];
        self.seed[1] ^= self.seed[2];
        self.seed[0] ^= self.seed[3];

        self.seed[2] ^= t;
        self.seed[3] = Xoshiro256plus::rol64(self.seed[3], 45);

        (result >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
}

pub struct SplitMix {
    seed: u64,
}


impl SplitMix {
    fn new(seed: u64) -> Self {
        SplitMix { seed }
    }
    /// from https://xoshiro.di.unimi.it/splitmix64.c
    fn next(&mut self) -> u64 {
        self.seed = self.seed.wrapping_add(0x9e3779b97f4a7c15);
        let mut z: u64 = self.seed;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}
