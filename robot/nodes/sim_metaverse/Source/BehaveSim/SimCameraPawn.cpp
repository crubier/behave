#include "SimCameraPawn.h"
#include "IoxBridge.h"
#include "Camera/CameraComponent.h"

ASimCameraPawn::ASimCameraPawn()
{
	PrimaryActorTick.bCanEverTick = true;

	RootComponent = CreateDefaultSubobject<USceneComponent>(TEXT("Root"));

	CameraComp = CreateDefaultSubobject<UCameraComponent>(TEXT("Camera"));
	CameraComp->SetupAttachment(RootComponent);
}

void ASimCameraPawn::BeginPlay()
{
	Super::BeginPlay();

	IoxSub = FIoxSimSubscriber::Create("behave/SimStatus");
	if (IoxSub)
	{
		UE_LOG(LogTemp, Log, TEXT("[BehaveSim] iceoryx2 subscriber ready"));
	}
	else
	{
		UE_LOG(LogTemp, Error, TEXT("[BehaveSim] failed to create iceoryx2 subscriber"));
	}
}

void ASimCameraPawn::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);

	if (!IoxSub)
	{
		return;
	}

	IoxPose Pose;
	if (IoxSub->Receive(Pose))
	{
		// Schema uses meters; UE5 uses centimetres.
		const FVector Location(Pose.x * 100.0, Pose.y * 100.0, Pose.z * 100.0);

		// Quaternion order: UE5 FQuat(X, Y, Z, W)
		const FQuat Rotation(Pose.qx, Pose.qy, Pose.qz, Pose.qw);

		SetActorLocationAndRotation(Location, Rotation.IsNormalized() ? Rotation : FQuat::Identity);
	}
}

void ASimCameraPawn::EndPlay(const EEndPlayReason::Type EndPlayReason)
{
	if (IoxSub)
	{
		delete IoxSub;
		IoxSub = nullptr;
	}

	Super::EndPlay(EndPlayReason);
}
