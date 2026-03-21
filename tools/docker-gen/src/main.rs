//! Generates optimized Dockerfiles for Rust workspace services using workspace-cache.

use std::env;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let service_name = current_dir
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("Could not determine service name from current directory")?;

    // Run from parent directory (services/)
    let parent_dir = current_dir
        .parent()
        .ok_or("Could not determine parent directory")?;

    let status = Command::new("workspace-cache")
        .current_dir(parent_dir)
        .arg("dockerfile")
        .arg("--bin")
        .arg(service_name)
        .arg("-o")
        .arg(format!("{}/Dockerfile", service_name))
        .arg("--fast")
        .status()?;

    if !status.success() {
        return Err(format!(
            "workspace-cache command failed with exit code: {:?}",
            status.code()
        )
        .into());
    }

    Ok(())
}
