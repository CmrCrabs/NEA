use crate::{cast_slice, renderer::WG_SIZE, scene::Scene};
use super::{SimData, Texture};

pub struct FourierTransform {
    h_ifft: PipelineFFT,
    v_ifft: PipelineFFT,
    permute_scale: PipelineFFT,
    copy_pingpong: PipelineFFT,
    pingpong1: super::Texture,
}

impl FourierTransform {
    pub fn new(
        scene: & Scene,
        simdata: &SimData,
        renderer: &crate::renderer::Renderer,
    ) -> Self {
        //TODO: potentially optimise
        let pingpong1 = Texture::new_fourier(scene.consts.sim.size, scene.consts.sim.size, wgpu::TextureFormat::Rgba32Float, renderer, "PingPong 1");
        let bind_group_layouts = &[&scene.consts_layout, &simdata.layout, &pingpong1.layout, &pingpong1.layout]; // layout is same for 0 and 1
        let h_ifft = PipelineFFT::new(bind_group_layouts, renderer, "H-Step IFFT", "fft::hstep_ifft");
        let v_ifft = PipelineFFT::new(bind_group_layouts, renderer, "V-Step IFFT", "fft::vstep_ifft");
        let permute_scale = PipelineFFT::new(bind_group_layouts, renderer, "Permute Scale", "fft::permute_scale");
        let copy_pingpong = PipelineFFT::new(bind_group_layouts, renderer, "Copy PingPong", "fft::copy_pingpong");

        Self {
            h_ifft,
            v_ifft,
            permute_scale,
            pingpong1,
            copy_pingpong
        }
    }

    // algorithm referenced from GPGPU TODO: credit
    pub fn ifft2d<'a>(
        &'a self, 
        renderer: &crate::renderer::Renderer,
        encoder: &'a mut wgpu::CommandEncoder,
        scene: &mut Scene,
        simdata: &SimData,
        pingpong0: &Texture,
    ) {
        let bind_groups = &[&scene.consts_bind_group, &simdata.bind_group, &pingpong0.bind_group, &self.pingpong1.bind_group];
        scene.consts.sim.pingpong = 0;
        scene.consts.sim.stage = 0;
        let wg_size = scene.consts.sim.size / WG_SIZE;

        for stage in 0..scene.consts.sim.size.ilog2() {
            scene.consts.sim.stage = stage;
            renderer.queue.write_buffer(&scene.consts_buf, 0, cast_slice(&[scene.consts]));
            self.h_ifft.compute(encoder, bind_groups, wg_size, wg_size);
            scene.consts.sim.pingpong = (scene.consts.sim.pingpong + 1) % 2;
        }

        for stage in 0..scene.consts.sim.size.ilog2() {
            scene.consts.sim.stage = stage;
            renderer.queue.write_buffer(&scene.consts_buf, 0, cast_slice(&[scene.consts]));
            self.v_ifft.compute(encoder, bind_groups, wg_size, wg_size);
            scene.consts.sim.pingpong = (scene.consts.sim.pingpong + 1) % 2;
        }

        self.copy_pingpong.compute(encoder, bind_groups, wg_size, wg_size);
        self.permute_scale.compute(encoder, bind_groups, wg_size, wg_size);
    }
}

pub struct PipelineFFT {
    pipeline: wgpu::ComputePipeline,
}

impl PipelineFFT {
    pub fn new(
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        renderer: &crate::renderer::Renderer,
        label: &str,
        entry_point: &str,
    ) -> Self {
        let pipeline_layout =
            renderer
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts,
                    push_constant_ranges: &[],
                    label: Some(label),
                });
        let pipeline =
            renderer
                .device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    entry_point: Some(entry_point),
                    layout: Some(&pipeline_layout),
                    module: &renderer.shader,
                    compilation_options: Default::default(),
                    cache: None,
                    label: Some(label),
                });
        Self {
            pipeline
        }
    }

    pub fn compute<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        bind_groups: &[&wgpu::BindGroup],
        x: u32,
        y: u32,
    ) {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            timestamp_writes: None,
            label: None,
        });

        pass.set_pipeline(&self.pipeline);
        for i in 0..bind_groups.len() {
            pass.set_bind_group(i as u32, bind_groups[i], &[]);
        }
        pass.dispatch_workgroups(x, y, 1);
    }
}
