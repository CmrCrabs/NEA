use super::SimData;
use crate::{cast_slice, engine::{scene::Scene, util::Texture}, WG_SIZE};
use shared::FFTData;
use std::mem;

pub struct FourierTransform {
    h_ifft: PipelineFFT,
    v_ifft: PipelineFFT,
    permute: PipelineFFT,
    pingpong1: Texture,
}

impl FourierTransform {
    pub fn new(
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
        scene: &Scene,
        simdata: &SimData,
    ) -> Self {
        let pingpong1 = Texture::new_storage(
            scene.consts.sim.size,
            scene.consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            device,
            "PingPong 1",
        );
        let bind_group_layouts = &[&simdata.layout, &pingpong1.layout, &pingpong1.layout]; // layout is same for 0 and 1
        let push_constant_ranges = &[wgpu::PushConstantRange {
            stages: wgpu::ShaderStages::COMPUTE,
            range: 0..mem::size_of::<FFTData>() as u32,
        }];

        let h_ifft = PipelineFFT::new(
            bind_group_layouts,
            push_constant_ranges,
            device,
            shader,
            "H-Step IFFT",
            "sim::fft::hstep_ifft",
        );
        let v_ifft = PipelineFFT::new(
            bind_group_layouts,
            push_constant_ranges,
            device,
            shader,
            "V-Step IFFT",
            "sim::fft::vstep_ifft",
        );
        let permute = PipelineFFT::new(
            bind_group_layouts,
            push_constant_ranges,
            device,
            shader,
            "Permute",
            "sim::fft::permute",
        );

        Self {
            h_ifft,
            v_ifft,
            permute,
            pingpong1,
        }
    }

    pub fn ifft2d<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        scene: &Scene,
        simdata: &SimData,
        pingpong0: &Texture,
        index: u32,
    ) {
        let bind_groups = &[
            &simdata.bind_group,
            &pingpong0.bind_group,
            &self.pingpong1.bind_group,
        ];
        let wg_size = scene.consts.sim.size / WG_SIZE;
        let mut data = FFTData {
            stage: 0,
            pingpong: 0,
        };

        for stage in 0..scene.consts.sim.size.ilog2() {
            data.stage = stage;
            self.h_ifft.compute(
                encoder,
                bind_groups,
                cast_slice(&[data]),
                &format!("H-Step {}, {}", stage, index),
                wg_size,
                wg_size,
            );
            data.pingpong = (data.pingpong + 1) % 2;
        }
        for stage in 0..scene.consts.sim.size.ilog2() {
            data.stage = stage;
            self.v_ifft.compute(
                encoder,
                bind_groups,
                cast_slice(&[data]),
                &format!("V-Step {}, {}", stage, index),
                wg_size,
                wg_size,
            );
            data.pingpong = (data.pingpong + 1) % 2;
        }

        self.permute.compute(
            encoder,
            bind_groups,
            cast_slice(&[data]),
            &format!("Permute {}", index),
            wg_size,
            wg_size,
        );
    }
}

// abstraction of the pipeline is just to make code significantly nicer
pub struct PipelineFFT {
    pipeline: wgpu::ComputePipeline,
}
impl PipelineFFT {
    pub fn new(
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        push_constant_ranges: &[wgpu::PushConstantRange],
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
        label: &str,
        entry_point: &str,
    ) -> Self {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts,
            push_constant_ranges,
            label: Some(label),
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            entry_point: Some(entry_point),
            layout: Some(&pipeline_layout),
            module: shader,
            compilation_options: Default::default(),
            cache: None,
            label: Some(label),
        });
        Self { pipeline }
    }

    pub fn compute<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        bind_groups: &[&wgpu::BindGroup],
        push_constants: &[u8],
        label: &str,
        x: u32,
        y: u32,
    ) {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            timestamp_writes: None,
            label: Some(label),
        });
        pass.set_pipeline(&self.pipeline);
        for (i, bind_group) in bind_groups.iter().enumerate() {
            pass.set_bind_group(i as _, *bind_group, &[]);
        }
        pass.set_push_constants(0, push_constants);
        pass.dispatch_workgroups(x, y, 1);
    }
}
