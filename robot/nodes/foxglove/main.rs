#[path = "../mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::foxglove::run() {
        eprintln!("[Foxglove] fatal: {e}");
        std::process::exit(1);
    }
}
