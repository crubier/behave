# Nodes

Each subfolder is an independent process (node) that communicates with other nodes via iceoryx2 topics.

## Node structure

Every node folder contains:

- `mod.rs` -- node logic (iceoryx2 setup, main loop)
- `main.rs` -- binary entry point
- `README.md` -- what the node does and which topics it uses
