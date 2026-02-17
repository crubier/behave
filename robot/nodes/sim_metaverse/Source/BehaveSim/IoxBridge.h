#pragma once

#include "CoreMinimal.h"

THIRD_PARTY_INCLUDES_START
#include "iox_bridge.h"
THIRD_PARTY_INCLUDES_END

/// Opaque handle to an iceoryx2 subscriber for SimStatus.
/// Implemented in IoxBridge.cpp to keep iceoryx2 internals out of UE5.
class FIoxSimSubscriber
{
public:
	static FIoxSimSubscriber* Create(const char* ServiceName);
	bool Receive(IoxPose& OutPose);
	~FIoxSimSubscriber();

private:
	FIoxSimSubscriber() = default;
	void* Impl = nullptr;
};
