cmake_minimum_required(VERSION 3.22)
project(iceoryx2_bridge LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)

# ── Fetch and build iceoryx2 automatically ───────────────────────
# This clones iceoryx2 v0.7.0 and builds the C++ bindings from source.
# Requires: Rust toolchain (cargo), CMake, C++ compiler.

include(FetchContent)
FetchContent_Declare(iceoryx2
    GIT_REPOSITORY https://github.com/eclipse-iceoryx/iceoryx2.git
    GIT_TAG        v0.7.0
    GIT_SHALLOW    TRUE
)

# Build iceoryx2 C++ bindings via its own CMakeLists.txt
FetchContent_MakeAvailable(iceoryx2)

# ── Bridge shared library ───────────────────────────────────────

add_library(iceoryx2_bridge SHARED iox_bridge.cpp)
target_link_libraries(iceoryx2_bridge PRIVATE iceoryx2-cxx::static-lib-cxx)
target_include_directories(iceoryx2_bridge PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

# ── Install: copy dylibs to ../iceoryx2/lib and header to ../iceoryx2/include
# so UE5's Build.cs can find them.

set(UE5_LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../iceoryx2/lib")
set(UE5_INC_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../iceoryx2/include")

install(TARGETS iceoryx2_bridge LIBRARY DESTINATION "${UE5_LIB_DIR}")
install(FILES iox_bridge.h DESTINATION "${UE5_INC_DIR}")

# Also install iceoryx runtime dependencies that the bridge links against
install(FILES
    $<TARGET_FILE:iceoryx_hoofs::iceoryx_hoofs>
    $<TARGET_FILE:iceoryx_platform::iceoryx_platform>
    DESTINATION "${UE5_LIB_DIR}"
)
