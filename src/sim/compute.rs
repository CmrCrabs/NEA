pub struct ComputePass {
    pub pipeline: wgpu::ComputePipeline,
}

impl ComputePass {
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
        label: &str,
        bind_groups: &[&wgpu::BindGroup],
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
        pass.dispatch_workgroups(x, y, 1);
    }
}
