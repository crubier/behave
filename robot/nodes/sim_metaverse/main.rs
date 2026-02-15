#[path = "../mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::sim_metaverse::run() {
        eprintln!("[SimMeta] fatal: {e}");
        std::process::exit(1);
    }
}
