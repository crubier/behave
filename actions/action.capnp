@0x90d4273e85b7f937;

# ── Args ────────────────────────────────────────────────────────
using import "sequence/sequence.capnp".SequenceArgs;
using import "fallback/fallback.capnp".FallbackArgs;
using import "takeoff/takeoff.capnp".TakeoffArgs;
using import "goto_waypoint/goto_waypoint.capnp".GotoWaypointArgs;
using import "return_home/return_home.capnp".ReturnHomeArgs;
using import "land/land.capnp".LandArgs;
using import "take_photo/take_photo.capnp".TakePhotoArgs;

# ── Result ──────────────────────────────────────────────────────
using import "sequence/sequence.capnp".SequenceResult;
using import "fallback/fallback.capnp".FallbackResult;
using import "takeoff/takeoff.capnp".TakeoffResult;
using import "goto_waypoint/goto_waypoint.capnp".GotoWaypointResult;
using import "return_home/return_home.capnp".ReturnHomeResult;
using import "land/land.capnp".LandResult;
using import "take_photo/take_photo.capnp".TakePhotoResult;

# ── State ───────────────────────────────────────────────────────
using import "sequence/sequence.capnp".SequenceState;
using import "fallback/fallback.capnp".FallbackState;
using import "takeoff/takeoff.capnp".TakeoffState;
using import "goto_waypoint/goto_waypoint.capnp".GotoWaypointState;
using import "return_home/return_home.capnp".ReturnHomeState;
using import "land/land.capnp".LandState;
using import "take_photo/take_photo.capnp".TakePhotoState;

# ── Input ───────────────────────────────────────────────────────
using import "sequence/sequence.capnp".SequenceInput;
using import "fallback/fallback.capnp".FallbackInput;
using import "takeoff/takeoff.capnp".TakeoffInput;
using import "goto_waypoint/goto_waypoint.capnp".GotoWaypointInput;
using import "return_home/return_home.capnp".ReturnHomeInput;
using import "land/land.capnp".LandInput;
using import "take_photo/take_photo.capnp".TakePhotoInput;

# ── Output ──────────────────────────────────────────────────────
using import "sequence/sequence.capnp".SequenceOutput;
using import "fallback/fallback.capnp".FallbackOutput;
using import "takeoff/takeoff.capnp".TakeoffOutput;
using import "goto_waypoint/goto_waypoint.capnp".GotoWaypointOutput;
using import "return_home/return_home.capnp".ReturnHomeOutput;
using import "land/land.capnp".LandOutput;
using import "take_photo/take_photo.capnp".TakePhotoOutput;

# ── Behavior Tree Node ──────────────────────────────────────────
#
# Each Action* struct is a recursive tree node.  Every node carries
# an id, an optional name, and a children list (empty for leaves).
# The unnamed union selects the concrete action / control-flow type.
#
# ActionArgs  -- mission-time parameters (what to do)
# ActionResult -- outcome after execution
# ActionState  -- internal runtime state
# ActionInput  -- real-time signals from GCS to robot
# ActionOutput -- real-time signals from robot to GCS

struct ActionArgs {
  id       @0 :UInt64;             # Unique node identifier
  name     @1 :Text;               # Optional human-readable label
  children @2 :List(ActionArgs);   # Child nodes (empty for leaves)

  union {
    sequence     @3 :SequenceArgs;
    fallback     @4 :FallbackArgs;
    takeoff      @5 :TakeoffArgs;
    gotoWaypoint @6 :GotoWaypointArgs;
    returnHome   @7 :ReturnHomeArgs;
    land         @8 :LandArgs;
    takePhoto    @9 :TakePhotoArgs;
  }
}

struct ActionResult {
  id       @0 :UInt64;
  children @1 :List(ActionResult);

  union {
    sequence     @2 :SequenceResult;
    fallback     @3 :FallbackResult;
    takeoff      @4 :TakeoffResult;
    gotoWaypoint @5 :GotoWaypointResult;
    returnHome   @6 :ReturnHomeResult;
    land         @7 :LandResult;
    takePhoto    @8 :TakePhotoResult;
  }
}

struct ActionState {
  id       @0 :UInt64;
  children @1 :List(ActionState);

  union {
    sequence     @2 :SequenceState;
    fallback     @3 :FallbackState;
    takeoff      @4 :TakeoffState;
    gotoWaypoint @5 :GotoWaypointState;
    returnHome   @6 :ReturnHomeState;
    land         @7 :LandState;
    takePhoto    @8 :TakePhotoState;
  }
}

struct ActionInput {
  id       @0 :UInt64;
  children @1 :List(ActionInput);

  union {
    sequence     @2 :SequenceInput;
    fallback     @3 :FallbackInput;
    takeoff      @4 :TakeoffInput;
    gotoWaypoint @5 :GotoWaypointInput;
    returnHome   @6 :ReturnHomeInput;
    land         @7 :LandInput;
    takePhoto    @8 :TakePhotoInput;
  }
}

struct ActionOutput {
  id       @0 :UInt64;
  children @1 :List(ActionOutput);

  union {
    sequence     @2 :SequenceOutput;
    fallback     @3 :FallbackOutput;
    takeoff      @4 :TakeoffOutput;
    gotoWaypoint @5 :GotoWaypointOutput;
    returnHome   @6 :ReturnHomeOutput;
    land         @7 :LandOutput;
    takePhoto    @8 :TakePhotoOutput;
  }
}
