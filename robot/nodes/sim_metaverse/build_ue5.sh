#!/usr/bin/env bash
set -euo pipefail

# Build the iceoryx2 bridge library and the UE5 BehaveSim module.
# Prerequisites: Rust toolchain, CMake, Xcode command-line tools, UE 5.7.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BRIDGE_DIR="$SCRIPT_DIR/Source/ThirdParty/iceoryx2_bridge"
LIB_DIR="$SCRIPT_DIR/Source/ThirdParty/iceoryx2/lib"
UPROJECT="$SCRIPT_DIR/BehaveSim.uproject"

echo "=== Building iceoryx2 bridge ==="

cmake -S "$BRIDGE_DIR" -B "$BRIDGE_DIR/build" -DCMAKE_BUILD_TYPE=Release -Wno-dev
cmake --build "$BRIDGE_DIR/build" -j"$(sysctl -n hw.ncpu)"
cmake --install "$BRIDGE_DIR/build" --prefix "$BRIDGE_DIR/build/_install"

# Create version symlinks for the iceoryx runtime dylibs
cd "$LIB_DIR"
ln -sf libiceoryx_hoofs.2.95.7.dylib libiceoryx_hoofs.2.dylib
ln -sf libiceoryx_platform.2.95.7.dylib libiceoryx_platform.2.dylib

echo "=== Building UE5 BehaveSim module ==="

"/Users/Shared/Epic Games/UE_5.7/Engine/Build/BatchFiles/Mac/Build.sh" \
    BehaveSim Mac Development "$UPROJECT"

echo "=== Done ==="
