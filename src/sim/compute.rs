pub struct InitialSpectraPass {
    pipeline: wgpu::ComputePipeline,
    buf: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl InitialSpectraPass {
   pub fn new(device: &wgpu::Device, shader: &wgpu::ShaderModule, ocean: &super::Ocean) -> Self {
        let buf = device.create_buffer(&wgpu::BufferDescriptor {
            size: std::mem::size_of::<shared::SimConstants>() as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            label: None,
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buf.as_entire_binding(),
            }],
            label: None,
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
            label: None,
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            entry_point: "initial_spectrum",
            layout: Some(&pipeline_layout),
            module: shader,
            label: None,
        });

        Self {
            buf,
            bind_group,
            pipeline,
        }
    } 
}
