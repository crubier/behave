#[path = "../mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::sense_sim::run() {
        eprintln!("[SenseSim] fatal: {e}");
        std::process::exit(1);
    }
}
