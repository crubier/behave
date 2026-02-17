#!/usr/bin/env bash
set -euo pipefail

# Clean all UE5 and iceoryx2 bridge build artifacts.
# After running this, rebuild with ./build_ue5.sh

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

echo "Cleaning UE5 module..."
rm -rf Binaries/ Intermediate/

echo "Cleaning iceoryx2 bridge..."
rm -rf Source/ThirdParty/iceoryx2_bridge/build
rm -rf Source/ThirdParty/iceoryx2/lib
rm -rf Source/ThirdParty/iceoryx2/include

echo "Cleaning UE5 cached settings and derived data..."
rm -rf Saved/ DerivedDataCache/

echo "Done. Rebuild with: ./build_ue5.sh"
