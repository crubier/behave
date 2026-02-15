#[path = "../nodes/mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::control::run() {
        eprintln!("[Control] fatal: {e}");
        std::process::exit(1);
    }
}
