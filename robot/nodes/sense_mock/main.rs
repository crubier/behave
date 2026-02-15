#[path = "../mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::sense_mock::run() {
        eprintln!("[SenseMock] fatal: {e}");
        std::process::exit(1);
    }
}
