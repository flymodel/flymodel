fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=proto");
    let targets = glob::glob("proto/**/*.proto")?;
    for target in targets {
        tonic_build::compile_protos(target?).unwrap();
    }
    Ok(())
}
