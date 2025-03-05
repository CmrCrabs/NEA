use super::sim::compute::ComputePass;
use super::sim::fft::FourierTransform;
use super::sim::SimData;
use crate::cast_slice;
use crate::scene::Scene;
use crate::sim::Cascade;

pub struct Simulation {
    pub simdata: SimData,
    pub cascade: Cascade,
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
        let cascade = Cascade::new(&device, &scene.consts);
        let simdata = SimData::new(&device, &scene.consts);

        let initial_spectra_pass = ComputePass::new(
            &[&scene.consts_layout, &simdata.layout, &cascade.stg_layout],
            &device,
            &shader,
            "Initial Spectra",
            "initial_spectra::main",
        );
        let butterfly_precompute_pass = ComputePass::new(
            &[&scene.consts_layout, &simdata.layout],
            &device,
            &shader,
            "Precompute Butterfly",
            "fft::precompute_butterfly",
        );
        let conjugates_pass = ComputePass::new(
            &[&scene.consts_layout, &cascade.stg_layout],
            &device,
            &shader,
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
            &device,
            &shader,
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
            &device,
            &shader,
            "Process Deltas",
            "process_deltas::main",
        );
        let fft = FourierTransform::new(&device, &shader, &scene, &simdata);

        simdata
            .gaussian_tex
            .write(&queue, cast_slice(&simdata.gaussian_noise.clone()), 16);

        Self {
            cascade,
            simdata,
            initial_spectra_pass,
            butterfly_precompute_pass,
            conjugates_pass,
            evolve_spectra_pass,
            process_deltas_pass,
            fft,
        }
    }
}
