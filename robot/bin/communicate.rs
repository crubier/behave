#[path = "../nodes/mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::communicate::run() {
        eprintln!("[Communicate] fatal: {e}");
        std::process::exit(1);
    }
}
