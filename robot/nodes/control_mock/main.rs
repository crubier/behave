#[path = "../mod.rs"]
mod nodes;

fn main() {
    if let Err(e) = nodes::control_mock::run() {
        eprintln!("[CtrlMock] fatal: {e}");
        std::process::exit(1);
    }
}
