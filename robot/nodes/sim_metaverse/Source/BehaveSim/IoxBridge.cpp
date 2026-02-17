#include "IoxBridge.h"

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

bool FIoxSimSubscriber::Receive(IoxPose& OutPose)
{
	if (!Impl)
	{
		return false;
	}
	return iox_sim_subscriber_receive(static_cast<IoxSimSubscriber*>(Impl), &OutPose);
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
