@0xc4a7b3e1d5f90812;

# ── Sense Status ───────────────────────────────────────────────
#
# Published by Sense nodes for all nodes to consume.
# Contains the fused navigation state.

struct SenseStatus {
  eastingM   @0 :Float64;
  northingM  @1 :Float64;
  altitudeM  @2 :Float64;
}
