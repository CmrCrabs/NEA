use spirv_builder::{SpirvBuilder, SpirvMetadata};

fn main() {
    SpirvBuilder::new("shaders", "spirv-unknown-spv1.5")
        .preserve_bindings(true)
        .spirv_metadata(SpirvMetadata::Full)
        .build()
        .unwrap();
}
