use super::sim::compute::ComputePass;
use super::sim::fft::FourierTransform;
use simdata::SimData;
use crate::cast_slice;
use crate::engine::scene::Scene;
use cascade::Cascade;

pub mod compute;
pub mod fft;
pub mod cascade;
pub mod simdata;


pub struct Simulation {
    pub simdata: SimData,
    pub cascade0: Cascade,
    pub cascade1: Cascade,
    pub cascade2: Cascade,
    pub butterfly_precompute_pass: ComputePass,
    pub initial_spectra_pass: ComputePass,
    pub conjugates_pass: ComputePass,
    pub evolve_spectra_pass: ComputePass,
    pub process_deltas_pass: ComputePass,
    pub fft: FourierTransform,
}

impl Simulation {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        shader: &wgpu::ShaderModule,
        scene: &Scene,
    ) -> Self {
        let simdata = SimData::new(device, &scene.consts);
        
        let cascade0 = Cascade::new(device, &scene.consts, "0");
        let cascade1 = Cascade::new(device, &scene.consts, "1");
        let cascade2 = Cascade::new(device, &scene.consts, "2");

        let push_constant_ranges = &[wgpu::PushConstantRange {
            stages: wgpu::ShaderStages::COMPUTE,
            range: 0..std::mem::size_of::<u32>() as u32,
        }];
        let initial_spectra_pass = ComputePass::new(
            &[&scene.consts_layout, &simdata.layout, &cascade0.layout],
            push_constant_ranges,
            device,
            shader,
            "Initial Spectra",
            "sim::initial_spectra::main",
        );
        let butterfly_precompute_pass = ComputePass::new(
            &[&scene.consts_layout, &simdata.layout],
            &[],
            device,
            shader,
            "Precompute Butterfly",
            "sim::fft::precompute_butterfly",
        );
        let conjugates_pass = ComputePass::new(
            &[&scene.consts_layout, &simdata.layout, &cascade0.layout],
            &[],
            device,
            shader,
            "Pack Conjugates",
            "sim::initial_spectra::pack_conjugates",
        );
        let evolve_spectra_pass = ComputePass::new(
            &[
                &scene.consts_layout,
                &cascade0.layout,
                &cascade0.h_displacement.layout,
                &cascade0.v_displacement.layout,
                &cascade0.h_slope.layout,
                &cascade0.jacobian.layout,
            ],
            &[],
            device,
            shader,
            "Evolve Spectra",
            "sim::evolve_spectra::main",
        );
        let process_deltas_pass = ComputePass::new(
            &[
                &scene.consts_layout,
                &cascade0.h_displacement.layout,
                &cascade0.v_displacement.layout,
                &cascade0.h_slope.layout,
                &cascade0.jacobian.layout,
                &cascade0.layout,
            ],
            &[],
            device,
            shader,
            "Process Deltas",
            "sim::process_deltas::main",
        );
        let fft = FourierTransform::new(device, shader, scene, &simdata);

        simdata
            .gaussian_tex
            .write(queue, cast_slice(&simdata.gaussian_noise.clone()), 16);

        Self {
            cascade0,
            cascade1,
            cascade2,
            simdata,
            initial_spectra_pass,
            butterfly_precompute_pass,
            conjugates_pass,
            evolve_spectra_pass,
            process_deltas_pass,
            fft,
        }
    }
    pub fn compute_cascade<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        cascade: &Cascade,
        scene: &mut Scene,
        workgroup_size: u32,
        index: u32,
    ) {
        self.evolve_spectra_pass.compute(
            encoder,
            "Evolve Spectra",
            &[
                &scene.consts_bind_group,
                &cascade.bind_group,
                &cascade.h_displacement.bind_group,
                &cascade.v_displacement.bind_group,
                &cascade.h_slope.bind_group,
                &cascade.jacobian.bind_group,
            ],
            workgroup_size,
            workgroup_size,
        );
        self.fft.ifft2d(
            encoder,
            scene,
            &self.simdata,
            &cascade.h_displacement,
            index,
        );
        self.fft.ifft2d(
            encoder,
            scene,
            &self.simdata,
            &cascade.v_displacement,
            index,
        );
        self.fft.ifft2d(
            encoder,
            scene,
            &self.simdata,
            &cascade.h_slope,
            index,
        );
        self.fft.ifft2d(
            encoder,
            scene,
            &self.simdata,
            &cascade.jacobian,
            index,
        );

        self.process_deltas_pass.compute(
            encoder,
            "Process Deltas",
            &[
                &scene.consts_bind_group,
                &cascade.h_displacement.bind_group,
                &cascade.v_displacement.bind_group,
                &cascade.h_slope.bind_group,
                &cascade.jacobian.bind_group,
                &cascade.bind_group,
            ],
            workgroup_size,
            workgroup_size,
        );
    }
    pub fn compute_initial<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        bind_groups: &[&wgpu::BindGroup],
        pc: u32,
        x: u32,
        y: u32,
    ) {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            timestamp_writes: None,
            label: Some("Initial Spectra"),
        });

        pass.set_pipeline(&self.initial_spectra_pass.pipeline);
        for (i, bind_group) in bind_groups.iter().enumerate() {
            pass.set_bind_group(i as _, *bind_group, &[]);
        }
        pass.set_push_constants(0, cast_slice(&[pc]));
        pass.dispatch_workgroups(x, y, 1);
        drop(pass);

        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            timestamp_writes: None,
            label: Some("Conjugates"),
        });

        pass.set_pipeline(&self.conjugates_pass.pipeline);
        for (i, bind_group) in bind_groups.iter().enumerate() {
            pass.set_bind_group(i as _, *bind_group, &[]);
        }
        pass.dispatch_workgroups(x, y, 1);
    }
}
