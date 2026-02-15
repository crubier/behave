#pragma once

// C FFI declarations for the sim_bridge Rust library.
// The library subscribes to the iceoryx2 "behave/SimRequest" service
// and provides the latest camera pose to UE5.

#include "CoreMinimal.h"

struct FSimRequest
{
	double X;   // meters
	double Y;   // meters
	double Z;   // meters
	double QW;  // quaternion scalar
	double QX;
	double QY;
	double QZ;
	uint64 Utime;
};

extern "C"
{
	/** Start the background iceoryx2 subscriber thread. */
	bool sim_bridge_init();

	/** Copy the latest pose into OutPose if new data arrived. Returns true when written. */
	bool sim_bridge_poll_pose(FSimRequest* OutPose);

	/** Stop the subscriber thread. */
	void sim_bridge_cleanup();
}
