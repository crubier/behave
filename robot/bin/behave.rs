#[path = "../nodes/mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::behave::run() {
        eprintln!("[Behave] fatal: {e}");
        std::process::exit(1);
    }
}
