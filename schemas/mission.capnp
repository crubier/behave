@0xa56f8f410d178796;

using import "/actions/action.capnp".ActionSpec;

# ── Mission envelope ────────────────────────────────────────────
#
# Wraps an ActionSpec (behavior tree) for transmission from
# the Communicate node to the Behave node.

struct Mission {
  id   @0 :UInt64;       # Unique mission identifier
  root @1 :ActionSpec;   # Behavior tree to execute
}
