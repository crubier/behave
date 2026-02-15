#include "SimCameraPawn.h"
#include "SimBridge.h"
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

	bBridgeInitialized = sim_bridge_init();
	if (bBridgeInitialized)
	{
		UE_LOG(LogTemp, Log, TEXT("[BehaveSim] sim_bridge initialised"));
	}
	else
	{
		UE_LOG(LogTemp, Error, TEXT("[BehaveSim] sim_bridge init FAILED"));
	}
}

void ASimCameraPawn::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);

	if (!bBridgeInitialized)
	{
		return;
	}

	FSimPose Pose;
	if (sim_bridge_poll_pose(&Pose))
	{
		// Schema uses meters; UE5 uses centimetres.
		const FVector Location(
			Pose.X * 100.0,
			Pose.Y * 100.0,
			Pose.Z * 100.0);

		// Quaternion order: UE5 FQuat(X, Y, Z, W)
		const FQuat Rotation(Pose.QX, Pose.QY, Pose.QZ, Pose.QW);

		SetActorLocationAndRotation(Location, Rotation.IsNormalized() ? Rotation : FQuat::Identity);
	}
}

void ASimCameraPawn::EndPlay(const EEndPlayReason::Type EndPlayReason)
{
	if (bBridgeInitialized)
	{
		sim_bridge_cleanup();
		bBridgeInitialized = false;
		UE_LOG(LogTemp, Log, TEXT("[BehaveSim] sim_bridge cleaned up"));
	}

	Super::EndPlay(EndPlayReason);
}
