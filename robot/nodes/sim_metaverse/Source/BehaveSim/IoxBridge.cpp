#include "IoxBridge.h"

THIRD_PARTY_INCLUDES_START
#include "iox_bridge.h"
THIRD_PARTY_INCLUDES_END

FIoxSimSubscriber* FIoxSimSubscriber::Create(const char* ServiceName)
{
	IoxSimSubscriber* RawSub = iox_sim_subscriber_create(ServiceName);
	if (!RawSub)
	{
		UE_LOG(LogTemp, Error, TEXT("[IoxBridge] failed to create iceoryx2 subscriber for %hs"), ServiceName);
		return nullptr;
	}

	auto* Self = new FIoxSimSubscriber();
	Self->Impl = RawSub;
	UE_LOG(LogTemp, Log, TEXT("[IoxBridge] subscribed to %hs via iceoryx2"), ServiceName);
	return Self;
}

bool FIoxSimSubscriber::Receive(FIoxPose& OutPose)
{
	if (!Impl)
	{
		return false;
	}

	IoxPose Pose;
	if (iox_sim_subscriber_receive(static_cast<IoxSimSubscriber*>(Impl), &Pose))
	{
		OutPose.X = Pose.x;
		OutPose.Y = Pose.y;
		OutPose.Z = Pose.z;
		OutPose.QW = Pose.qw;
		OutPose.QX = Pose.qx;
		OutPose.QY = Pose.qy;
		OutPose.QZ = Pose.qz;
		OutPose.Utime = Pose.utime;
		return true;
	}
	return false;
}

FIoxSimSubscriber::~FIoxSimSubscriber()
{
	if (Impl)
	{
		iox_sim_subscriber_destroy(static_cast<IoxSimSubscriber*>(Impl));
		Impl = nullptr;
		UE_LOG(LogTemp, Log, TEXT("[IoxBridge] subscriber destroyed"));
	}
}
