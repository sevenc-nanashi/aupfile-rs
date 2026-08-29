use std::env;
use std::error::Error;
use std::fs::File;

use aupfile::AviUtlProject;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let input = args
        .next()
        .ok_or("usage: cargo run --example export_exo -- <project.aup> <scene> <output.exo>")?;
    let scene = args.next().ok_or("scene is required")?.parse::<u32>()?;
    let output = args.next().ok_or("output path is required")?;
    if args.next().is_some() {
        return Err("too many arguments".into());
    }

    let mut project = AviUtlProject::open(input)?;
    let edit_handle = project.edit_handle.clone();
    let exedit = project
        .decode_exedit()?
        .ok_or("ExEdit project was not found")?;
    let exo = exedit.export_object(scene, &edit_handle)?;
    exo.write(File::create(output)?)?;
    Ok(())
}
