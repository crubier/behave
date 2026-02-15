@0xf5d780766c879f63;

# ── Sequence ────────────────────────────────────────────────────
# Ticks children left-to-right; succeeds only if ALL children succeed.
# Children are carried by ActionSpec.children.

struct SequenceArgs {
  # Per-sequence configuration (reserved for future use)
}

struct SequenceResult {
  success             @0 :Bool;
  childrenCompleted   @1 :UInt32;  # Number of children that ran successfully
  failedAtIndex       @2 :Int32;   # Index of first failed child (-1 if none)
}

enum ChildStatus {
  pending   @0;
  running   @1;
  succeeded @2;
  failed    @3;
  skipped   @4;
}

struct ChildState {
  nodeId @0 :UInt64;
  status @1 :ChildStatus;
}

struct SequenceState {
  currentIndex   @0 :UInt32;           # Index of the currently executing child
  childStates    @1 :List(ChildState); # Status of each child
}

struct SequenceInput {
  # Real-time signals from GCS (reserved for future use)
}

struct SequenceOutput {
  progressPct @0 :Float64;  # 0..100 based on children completed
  currentStep @1 :UInt32;
  totalSteps  @2 :UInt32;
}
