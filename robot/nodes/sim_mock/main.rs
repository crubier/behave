#[path = "../mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::sim_mock::run() {
        eprintln!("[SimMock] fatal: {e}");
        std::process::exit(1);
    }
}
