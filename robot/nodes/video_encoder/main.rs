#[path = "../mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::video_encoder::run() {
        eprintln!("[VideoEnc] fatal: {e}");
        std::process::exit(1);
    }
}
