//! Robot launcher -- spawns each node as its own process.
//!
//! Nodes communicate via iceoryx2 shared-memory IPC, which works
//! identically across processes.

use std::process::{Child, Command};

use log::{error, info};

fn main() {
    behave::logging::init("Robot");

    info!("=== Robot starting ===");

    let exe = std::env::current_exe().expect("failed to get current exe path");
    let bin_dir = exe.parent().expect("failed to get binary directory");

    let nodes = ["node-sense", "node-control", "node-behave", "node-communicate"];

    let mut children: Vec<(&str, Child)> = Vec::new();
    for name in &nodes {
        let path = bin_dir.join(name);
        match Command::new(&path)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
        {
            Ok(child) => {
                info!("spawned {name} (pid {})", child.id());
                children.push((name, child));
            }
            Err(e) => {
                error!("failed to spawn {name} at {}: {e}", path.display());
                for (n, mut c) in children {
                    error!("killing {n}");
                    let _ = c.kill();
                }
                std::process::exit(1);
            }
        }
    }

    info!("=== all {} nodes launched ===", children.len());

    for (name, mut child) in children {
        match child.wait() {
            Ok(status) => {
                info!("{name} exited with {status}");
            }
            Err(e) => {
                error!("error waiting for {name}: {e}");
            }
        }
    }
}
