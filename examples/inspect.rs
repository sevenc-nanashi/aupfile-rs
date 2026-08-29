use std::env;
use std::error::Error;

use aupfile::{AviUtlProject, FilterProject};

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or("usage: cargo run --example inspect -- <project.aup>")?;
    let project = AviUtlProject::open(path)?;

    println!(
        "{}x{}, {}/{}, {} frames",
        project.edit_handle.width,
        project.edit_handle.height,
        project.edit_handle.video_rate,
        project.edit_handle.video_scale,
        project.edit_handle.frames.len()
    );
    for filter in &project.filter_projects {
        let kind = match filter {
            FilterProject::Raw(_) => "raw",
            FilterProject::ExEdit(_) => "ExEdit",
        };
        println!("filter: {} ({kind})", filter.name());
    }
    Ok(())
}
