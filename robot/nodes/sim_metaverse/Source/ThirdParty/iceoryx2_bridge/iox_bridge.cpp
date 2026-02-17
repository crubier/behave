// Thin C++ wrapper around iceoryx2 subscriber.
// Built with CMake where iceoryx2 C++ headers work correctly.

#include "iox_bridge.h"
#include "iox2/iceoryx2.hpp"

using namespace iox2;

// Must match the Rust struct behave::topics::sim::status::SimStatus
// and behave::topics::sim::CameraPose exactly (same field order, same sizes).
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

extern "C" {

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

} // extern "C"
