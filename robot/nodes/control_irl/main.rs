#[path = "../mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::control_irl::run() {
        eprintln!("[ControlIRL] fatal: {e}");
        std::process::exit(1);
    }
}
