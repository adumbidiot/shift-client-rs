use std::path::Path;

fn main() -> anyhow::Result<()> {
    let out_dir = Path::new("src/generated");

    std::fs::create_dir_all(out_dir)?;

    prost_build::Config::new()
        .out_dir(out_dir)
        .compile_protos(&["proto/launcher.proto"], &["proto"])?;

    Ok(())
}
