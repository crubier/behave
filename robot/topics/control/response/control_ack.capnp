@0x8cca68bbe2c189e4;

# ── Control Acknowledgment ──────────────────────────────────────
#
# Sent from Control -> Behave after each command.

struct ControlAck {
  commandId @0 :UInt64;
  success   @1 :Bool;
  message   @2 :Text;
}
