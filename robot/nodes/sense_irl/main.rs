#[path = "../mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::sense_irl::run() {
        eprintln!("[SenseIRL] fatal: {e}");
        std::process::exit(1);
    }
}
