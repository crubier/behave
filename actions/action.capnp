@0x90d4273e85b7f937;

using import "sequence/sequence.capnp".Sequence;
using import "fallback/fallback.capnp".Fallback;
using import "takeoff/takeoff.capnp".Takeoff;
using import "goto_waypoint/goto_waypoint.capnp".GotoWaypoint;
using import "return_home/return_home.capnp".ReturnHome;
using import "land/land.capnp".Land;
using import "take_photo/take_photo.capnp".TakePhoto;

# ── Behavior Tree Node ──────────────────────────────────────────
#
# ActionSpec is the recursive tree node.  Every node carries an id,
# an optional name, and a children list (empty for leaf nodes).
# The unnamed union selects the concrete action / control-flow type.

struct ActionSpec {
  id       @0 :UInt64;            # Unique node identifier
  name     @1 :Text;              # Optional human-readable label
  children @2 :List(ActionSpec);  # Child nodes (empty for leaves)

  union {
    # ── Composite (branch) nodes ──────────────────────────────
    sequence     @3 :Sequence;
    fallback     @4 :Fallback;

    # ── Leaf (action) nodes ───────────────────────────────────
    takeoff      @5 :Takeoff;
    gotoWaypoint @6 :GotoWaypoint;
    returnHome   @7 :ReturnHome;
    land         @8 :Land;
    takePhoto    @9 :TakePhoto;
  }
}
