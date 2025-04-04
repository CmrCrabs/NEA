use spirv_builder::{MetadataPrintout, SpirvBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SpirvBuilder::new("shaders", "spirv-unknown-spv1.5")
        .capability(spirv_builder::Capability::StorageImageExtendedFormats)
        //.capability(spirv_builder::Capability::StoragePushConstant8)
        .print_metadata(MetadataPrintout::Full)
        .build()?;
    Ok(())
}
