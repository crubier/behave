#include "SimCameraPawn.h"
#include "IoxBridge.h"
#include "Camera/CameraComponent.h"
#include "Components/SceneCaptureComponent2D.h"
#include "Engine/TextureRenderTarget2D.h"

THIRD_PARTY_INCLUDES_START
#include "iox_bridge.h"
THIRD_PARTY_INCLUDES_END

ASimCameraPawn::ASimCameraPawn()
{
	PrimaryActorTick.bCanEverTick = true;

	RootComponent = CreateDefaultSubobject<USceneComponent>(TEXT("Root"));

	CameraComp = CreateDefaultSubobject<UCameraComponent>(TEXT("Camera"));
	CameraComp->SetupAttachment(RootComponent);

	SceneCapture = CreateDefaultSubobject<USceneCaptureComponent2D>(TEXT("SceneCapture"));
	SceneCapture->SetupAttachment(CameraComp);
	SceneCapture->CaptureSource = ESceneCaptureSource::SCS_FinalColorLDR;
}

void ASimCameraPawn::BeginPlay()
{
	Super::BeginPlay();

	// Create render target at capture resolution
	RenderTarget = NewObject<UTextureRenderTarget2D>(this);
	RenderTarget->InitAutoFormat(IOX_FRAME_WIDTH, IOX_FRAME_HEIGHT);
	RenderTarget->RenderTargetFormat = ETextureRenderTargetFormat::RTF_RGBA8;
	SceneCapture->TextureTarget = RenderTarget;

	// iceoryx2 subscriber for pose
	IoxSub = FIoxSimSubscriber::Create("behave/SimStatus");
	if (IoxSub)
	{
		UE_LOG(LogTemp, Log, TEXT("[BehaveSim] pose subscriber ready"));
	}

	// iceoryx2 publisher for raw frames
	FramePub = iox_frame_publisher_create("behave/video/CameraRawFrame");
	if (FramePub)
	{
		UE_LOG(LogTemp, Log, TEXT("[BehaveSim] frame publisher ready (%dx%d BGRA)"), IOX_FRAME_WIDTH, IOX_FRAME_HEIGHT);
	}
}

void ASimCameraPawn::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);

	// Update pose from iceoryx2
	if (IoxSub)
	{
		IoxPose Pose;
		if (IoxSub->Receive(Pose))
		{
			const FVector Location(Pose.x * 100.0, Pose.y * 100.0, Pose.z * 100.0);
			const FQuat Rotation(Pose.qx, Pose.qy, Pose.qz, Pose.qw);
			SetActorLocationAndRotation(Location, Rotation.IsNormalized() ? Rotation : FQuat::Identity);
		}
	}

	// Read back rendered frame and publish
	if (FramePub && RenderTarget)
	{
		FTextureRenderTargetResource* Resource = RenderTarget->GameThread_GetRenderTargetResource();
		if (Resource)
		{
			TArray<FColor> Pixels;
			Resource->ReadPixels(Pixels);

			if (Pixels.Num() == IOX_FRAME_WIDTH * IOX_FRAME_HEIGHT)
			{
				uint64_t Utime = static_cast<uint64_t>(
					FPlatformTime::ToMilliseconds64(FPlatformTime::Cycles64()) * 1000.0);
				iox_frame_publisher_send(FramePub, Utime,
					reinterpret_cast<const uint8_t*>(Pixels.GetData()));
			}
		}
	}
}

void ASimCameraPawn::EndPlay(const EEndPlayReason::Type EndPlayReason)
{
	if (FramePub)
	{
		iox_frame_publisher_destroy(FramePub);
		FramePub = nullptr;
	}

	if (IoxSub)
	{
		delete IoxSub;
		IoxSub = nullptr;
	}

	Super::EndPlay(EndPlayReason);
}
