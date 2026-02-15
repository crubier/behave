#[path = "../nodes/mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::sense::run() {
        eprintln!("[Sense] fatal: {e}");
        std::process::exit(1);
    }
}
