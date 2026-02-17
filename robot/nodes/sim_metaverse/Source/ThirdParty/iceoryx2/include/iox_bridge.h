// Thin C wrapper around iceoryx2 C++ subscriber for UE5 integration.
// Built as a separate shared library to avoid header conflicts with UE5.

#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>

typedef struct IoxSimSubscriber IoxSimSubscriber;

typedef struct IoxPose {
    double x, y, z;
    double qw, qx, qy, qz;
    uint64_t utime;
} IoxPose;

IoxSimSubscriber* iox_sim_subscriber_create(const char* service_name);
bool iox_sim_subscriber_receive(IoxSimSubscriber* sub, IoxPose* out_pose);
void iox_sim_subscriber_destroy(IoxSimSubscriber* sub);

#ifdef __cplusplus
}
#endif
