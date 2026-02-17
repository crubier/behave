#pragma once

#include "CoreMinimal.h"

/// Pose data received via iceoryx2 shared memory.
struct FIoxPose
{
	double X, Y, Z;
	double QW, QX, QY, QZ;
	uint64 Utime;
};

/// Opaque handle to an iceoryx2 subscriber for SimStatus.
/// Implemented in IoxBridge.cpp to keep iceoryx2 headers out of UE5 headers.
class FIoxSimSubscriber
{
public:
	static FIoxSimSubscriber* Create(const char* ServiceName);
	bool Receive(FIoxPose& OutPose);
	~FIoxSimSubscriber();

private:
	FIoxSimSubscriber() = default;
	void* Impl = nullptr;
};
