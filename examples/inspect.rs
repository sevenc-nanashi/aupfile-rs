use std::env;
use std::error::Error;

use aupfile::AviUtlProject;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or("usage: cargo run --example inspect -- <project.aup>")?;
    let mut project = AviUtlProject::open(path)?;
    if let Err(e) = project.decode_exedit() {
        eprintln!("failed to decode ExEdit project: {e}");
    }

    println!(
        "{}x{}, {}/{}, {} frames",
        project.edit_handle.width,
        project.edit_handle.height,
        project.edit_handle.video_rate,
        project.edit_handle.video_scale,
        project.edit_handle.frames.len()
    );
    for filter in &project.filter_projects {
        println!("filter: {}", filter.name(),);
    }
    Ok(())
}
