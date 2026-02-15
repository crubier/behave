//! Generic runmode launcher.
//!
//! Usage: runmode <mode.yaml>
//! Reads the config and spawns all listed node binaries as separate processes.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

use indexmap::IndexMap;
use log::{error, info};

#[derive(serde::Deserialize)]
struct ModeConfig {
    name: String,
    #[serde(default)]
    description: String,
    nodes: IndexMap<String, Option<HashMap<String, serde_yaml::Value>>>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config_path = match args.get(1) {
        Some(p) => PathBuf::from(p),
        None => {
            eprintln!("Usage: runmode <mode.json>");
            std::process::exit(1);
        }
    };

    let config = load_config(&config_path);

    behave::logging::init(&config.name);
    info!("=== {} ===", config.name);
    if !config.description.is_empty() {
        info!("{}", config.description);
    }

    launch(&config);
}

fn load_config(path: &Path) -> ModeConfig {
    let data = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read runmode config {}: {e}", path.display()));
    serde_yaml::from_str(&data)
        .unwrap_or_else(|e| panic!("failed to parse mode config {}: {e}", path.display()))
}

fn launch(config: &ModeConfig) {
    let exe = std::env::current_exe().expect("failed to get current exe path");
    let bin_dir = exe.parent().expect("failed to get binary directory");

    info!("launching {} nodes", config.nodes.len());

    let mut children: Vec<(String, Child)> = Vec::new();
    for (name, params) in &config.nodes {
        let bin_name = format!("node-{name}");
        let path = bin_dir.join(&bin_name);
        let mut cmd = Command::new(&path);
        cmd.stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit());

        // Pass node parameters as environment variables (BEHAVE_<KEY>=<VALUE>)
        if let Some(params) = params {
            for (key, value) in params {
                let env_key = format!("BEHAVE_{}", key.to_uppercase());
                let env_val = match value {
                    serde_yaml::Value::Bool(b) => b.to_string(),
                    serde_yaml::Value::Number(n) => n.to_string(),
                    serde_yaml::Value::String(s) => s.clone(),
                    other => format!("{other:?}"),
                };
                cmd.env(&env_key, &env_val);
                info!("  {name}: {env_key}={env_val}");
            }
        }

        match cmd.spawn() {
            Ok(child) => {
                info!("spawned {name} (pid {})", child.id());
                children.push((name.to_string(), child));
            }
            Err(e) => {
                error!("failed to spawn {name} ({bin_name}) at {}: {e}", path.display());
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
            Ok(status) => info!("{name} exited with {status}"),
            Err(e) => error!("error waiting for {name}: {e}"),
        }
    }
}
