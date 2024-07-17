fn main() -> Result<(), Box<dyn std::error::Error>> {
    capnpc::CompilerCommand::new()
        .file("src\\hello_world.capnp")
        .run()?;
    Ok(())
}
