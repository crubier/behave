// Thin C wrapper around iceoryx2 C++ pub/sub for UE5 integration.
// Built as a separate shared library to avoid header conflicts with UE5.

#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>

// ── SimStatus subscriber ────────────────────────────────────────

typedef struct IoxSimSubscriber IoxSimSubscriber;

typedef struct IoxPose {
    double x, y, z;
    double qw, qx, qy, qz;
    uint64_t utime;
} IoxPose;

IoxSimSubscriber* iox_sim_subscriber_create(const char* service_name);
bool iox_sim_subscriber_receive(IoxSimSubscriber* sub, IoxPose* out_pose);
void iox_sim_subscriber_destroy(IoxSimSubscriber* sub);

// ── CameraRawFrame publisher ────────────────────────────────────

#define IOX_FRAME_WIDTH  640
#define IOX_FRAME_HEIGHT 480
#define IOX_FRAME_BPP    4
#define IOX_FRAME_SIZE   (IOX_FRAME_WIDTH * IOX_FRAME_HEIGHT * IOX_FRAME_BPP)

typedef struct IoxFramePublisher IoxFramePublisher;

IoxFramePublisher* iox_frame_publisher_create(const char* service_name);
bool iox_frame_publisher_send(IoxFramePublisher* pub, uint64_t utime, const uint8_t* bgra_data);
void iox_frame_publisher_destroy(IoxFramePublisher* pub);

#ifdef __cplusplus
}
#endif
