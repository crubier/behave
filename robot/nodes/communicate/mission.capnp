@0xa56f8f410d178796;

using import "/actions/action.capnp".ActionArgs;

# ── Mission envelope ────────────────────────────────────────────
#
# Wraps an ActionArgs (behavior tree) for transmission from
# the Communicate node to the Behave node.

struct Mission {
  id   @0 :UInt64;       # Unique mission identifier
  root @1 :ActionArgs;   # Behavior tree to execute
}
