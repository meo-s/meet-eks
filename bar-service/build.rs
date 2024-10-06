use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let descriptor_path = PathBuf::from(env::var("OUT_DIR")?);

    tonic_build::configure()
        .file_descriptor_set_path(descriptor_path.join("service.bar.descriptor.bin"))
        .compile_protos(&["service/bar.proto"], &["../proto"])?;

    Ok(())
}
