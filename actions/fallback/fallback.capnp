@0xc18dfbdef887c496;

# ── Fallback ────────────────────────────────────────────────────
# Ticks children left-to-right; succeeds if ANY child succeeds.
# Children are carried by ActionSpec.children.

using import "../sequence/sequence.capnp".ChildState;

struct FallbackArgs {
  # Per-fallback configuration (reserved for future use)
}

struct FallbackResult {
  success           @0 :Bool;
  succeededAtIndex  @1 :Int32;   # Index of first successful child (-1 if none)
  childrenAttempted @2 :UInt32;  # Number of children tried before success/failure
}

struct FallbackState {
  currentIndex   @0 :UInt32;           # Index of the currently executing child
  childStates    @1 :List(ChildState); # Status of each child
}

struct FallbackInput {
  # Real-time signals from GCS (reserved for future use)
}

struct FallbackOutput {
  progressPct  @0 :Float64;  # 0..100 based on children attempted
  currentStep  @1 :UInt32;
  totalSteps   @2 :UInt32;
}
