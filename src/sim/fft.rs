use super::{SimData, Texture};
use crate::{cast_slice, renderer::WG_SIZE, scene::Scene};
use shared::FFTData;
use std::mem;

pub struct FourierTransform {
    h_ifft: PipelineFFT,
    v_ifft: PipelineFFT,
    permute_scale: PipelineFFT,
    pingpong1: super::Texture,
}

impl FourierTransform {
    pub fn new(scene: &Scene, simdata: &SimData, renderer: &crate::renderer::Renderer) -> Self {
        //TODO: potentially optimise
        let pingpong1 = Texture::new_fourier(
            scene.consts.sim.size,
            scene.consts.sim.size,
            wgpu::TextureFormat::Rgba32Float,
            renderer,
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
            renderer,
            "H-Step IFFT",
            "fft::hstep_ifft",
        );
        let v_ifft = PipelineFFT::new(
            bind_group_layouts,
            push_constant_ranges,
            renderer,
            "V-Step IFFT",
            "fft::vstep_ifft",
        );
        let permute_scale = PipelineFFT::new(
            bind_group_layouts,
            push_constant_ranges,
            renderer,
            "Permute Scale",
            "fft::permute_scale",
        );

        Self {
            h_ifft,
            v_ifft,
            permute_scale,
            pingpong1,
        }
    }

    // algorithm referenced from GPGPU TODO: credit
    pub fn ifft2d<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        scene: &Scene,
        simdata: &SimData,
        pingpong0: &Texture,
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
                format!("H-Step {}", stage).as_str(),
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
                format!("V-Step {}", stage).as_str(),
                wg_size,
                wg_size,
            );
            data.pingpong = (data.pingpong + 1) % 2;
        }

        self.permute_scale.compute(
            encoder,
            bind_groups,
            cast_slice(&[data]),
            "Permute Scale",
            wg_size,
            wg_size,
        );
    }
}

pub struct PipelineFFT {
    pipeline: wgpu::ComputePipeline,
}

impl PipelineFFT {
    pub fn new(
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        push_constant_ranges: &[wgpu::PushConstantRange],
        renderer: &crate::renderer::Renderer,
        label: &str,
        entry_point: &str,
    ) -> Self {
        let pipeline_layout =
            renderer
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts,
                    push_constant_ranges,
                    label: Some(label),
                });
        let pipeline = renderer
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                entry_point: Some(entry_point),
                layout: Some(&pipeline_layout),
                module: &renderer.shader,
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
        //TODO: make 1 pass instead

        pass.set_pipeline(&self.pipeline);
        for i in 0..bind_groups.len() {
            pass.set_bind_group(i as u32, bind_groups[i], &[]);
        }
        pass.set_push_constants(0, push_constants);
        pass.dispatch_workgroups(x, y, 1);
    }
}
