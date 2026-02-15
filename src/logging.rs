//! Shared logging setup for all robot nodes.
//!
//! Each node calls `init("NodeName")` to get timestamped, prefixed log output.

use std::io::Write;

pub fn init(node_name: &str) {
    let prefix = node_name.to_string();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(move |buf, record| {
            let ts = buf.timestamp_millis();
            writeln!(buf, "{ts} [{prefix:>11}] {}", record.args())
        })
        .init();
}
