#[path = "../mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::control_sim::run() {
        eprintln!("[ControlSim] fatal: {e}");
        std::process::exit(1);
    }
}
