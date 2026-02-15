#[path = "../mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::mock_gcs::run() {
        eprintln!("[MockGCS] fatal: {e}");
        std::process::exit(1);
    }
}
