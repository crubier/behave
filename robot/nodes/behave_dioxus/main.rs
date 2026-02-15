#[path = "../mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::behave_dioxus::run() {
        eprintln!("[BehaveDioxus] fatal: {e}");
        std::process::exit(1);
    }
}
