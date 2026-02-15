#[path = "../mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::gcs_mock::run() {
        eprintln!("[GcsMock] fatal: {e}");
        std::process::exit(1);
    }
}
