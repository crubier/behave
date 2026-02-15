#[path = "../mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::control_metaverse::run() {
        eprintln!("[CtrlMeta] fatal: {e}");
        std::process::exit(1);
    }
}
