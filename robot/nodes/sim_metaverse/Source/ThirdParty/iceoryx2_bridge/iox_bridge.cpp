// Thin C++ wrapper around iceoryx2 pub/sub.
// Built with CMake where iceoryx2 C++ headers work correctly.

#include "iox_bridge.h"
#include "iox2/iceoryx2.hpp"

#include <cstring>

using namespace iox2;

// ── SimStatus (must match Rust behave::topics::sim::status::SimStatus) ──

struct IoxCameraPose {
    double x, y, z;
    double qw, qx, qy, qz;
};

struct IoxSimStatus {
    static constexpr const char* IOX2_TYPE_NAME = "behave::topics::sim::status::SimStatus";
    IoxCameraPose pose;
    uint64_t utime;
};

struct IoxSimSubscriber {
    Node<ServiceType::Ipc> node;
    Subscriber<ServiceType::Ipc, IoxSimStatus, void> sub;

    IoxSimSubscriber(Node<ServiceType::Ipc>&& n, Subscriber<ServiceType::Ipc, IoxSimStatus, void>&& s)
        : node(std::move(n)), sub(std::move(s)) {}
};

// ── CameraRawFrame (must match Rust behave::topics::video::camera_raw_frame::CameraRawFrame) ──

struct IoxCameraRawFrame {
    static constexpr const char* IOX2_TYPE_NAME = "behave::topics::video::camera_raw_frame::CameraRawFrame";
    uint32_t width;
    uint32_t height;
    uint32_t stride;
    uint64_t utime;
    uint8_t data[IOX_FRAME_SIZE];
};

struct IoxFramePublisher {
    Node<ServiceType::Ipc> node;
    Publisher<ServiceType::Ipc, IoxCameraRawFrame, void> pub;

    IoxFramePublisher(Node<ServiceType::Ipc>&& n, Publisher<ServiceType::Ipc, IoxCameraRawFrame, void>&& p)
        : node(std::move(n)), pub(std::move(p)) {}
};

// ── C API ───────────────────────────────────────────────────────

extern "C" {

// -- SimStatus subscriber --

IoxSimSubscriber* iox_sim_subscriber_create(const char* service_name) {
    auto node_result = NodeBuilder().create<ServiceType::Ipc>();
    if (node_result.has_error()) return nullptr;
    auto node = std::move(node_result.value());

    auto name_result = ServiceName::create(service_name);
    if (name_result.has_error()) return nullptr;

    auto service_result = node.service_builder(name_result.value())
        .publish_subscribe<IoxSimStatus>()
        .open_or_create();
    if (service_result.has_error()) return nullptr;

    auto sub_result = service_result.value().subscriber_builder().create();
    if (sub_result.has_error()) return nullptr;

    return new IoxSimSubscriber(std::move(node), std::move(sub_result.value()));
}

bool iox_sim_subscriber_receive(IoxSimSubscriber* sub, IoxPose* out) {
    if (!sub || !out) return false;

    bool got = false;
    while (true) {
        auto result = sub->sub.receive();
        if (result.has_error()) break;
        if (!result.value().has_value()) break;
        const auto& p = result.value().value().payload();
        out->x = p.pose.x;  out->y = p.pose.y;  out->z = p.pose.z;
        out->qw = p.pose.qw; out->qx = p.pose.qx;
        out->qy = p.pose.qy; out->qz = p.pose.qz;
        out->utime = p.utime;
        got = true;
    }
    return got;
}

void iox_sim_subscriber_destroy(IoxSimSubscriber* sub) {
    delete sub;
}

// -- CameraRawFrame publisher --

IoxFramePublisher* iox_frame_publisher_create(const char* service_name) {
    auto node_result = NodeBuilder().create<ServiceType::Ipc>();
    if (node_result.has_error()) return nullptr;
    auto node = std::move(node_result.value());

    auto name_result = ServiceName::create(service_name);
    if (name_result.has_error()) return nullptr;

    auto service_result = node.service_builder(name_result.value())
        .publish_subscribe<IoxCameraRawFrame>()
        .open_or_create();
    if (service_result.has_error()) return nullptr;

    auto pub_result = service_result.value().publisher_builder().create();
    if (pub_result.has_error()) return nullptr;

    return new IoxFramePublisher(std::move(node), std::move(pub_result.value()));
}

bool iox_frame_publisher_send(IoxFramePublisher* pub, uint64_t utime, const uint8_t* bgra_data) {
    if (!pub || !bgra_data) return false;

    auto sample_result = pub->pub.loan_uninit();
    if (sample_result.has_error()) return false;

    auto& frame = sample_result.value();
    IoxCameraRawFrame payload;
    payload.width = IOX_FRAME_WIDTH;
    payload.height = IOX_FRAME_HEIGHT;
    payload.stride = IOX_FRAME_WIDTH * IOX_FRAME_BPP;
    payload.utime = utime;
    std::memcpy(payload.data, bgra_data, IOX_FRAME_SIZE);

    send(frame.write_payload(payload)).value();
    return true;
}

void iox_frame_publisher_destroy(IoxFramePublisher* pub) {
    delete pub;
}

} // extern "C"
