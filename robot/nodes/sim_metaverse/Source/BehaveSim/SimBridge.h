#pragma once

// Flat pose packet received over UDP from sim_metaverse (64 bytes, little-endian).
// Must match the Rust UdpPosePacket struct exactly.

#include "CoreMinimal.h"

#pragma pack(push, 1)
struct FSimPosePacket
{
	double X;   // meters
	double Y;   // meters
	double Z;   // meters
	double QW;  // quaternion scalar
	double QX;
	double QY;
	double QZ;
	uint64 Utime;  // microseconds
};
#pragma pack(pop)

static_assert(sizeof(FSimPosePacket) == 64, "FSimPosePacket must be 64 bytes");
