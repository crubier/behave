#[path = "../mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::sense_metaverse::run() {
        eprintln!("[SenseMeta] fatal: {e}");
        std::process::exit(1);
    }
}
