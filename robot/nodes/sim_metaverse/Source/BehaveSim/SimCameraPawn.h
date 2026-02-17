#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Pawn.h"
#include "SimCameraPawn.generated.h"

class FIoxSimSubscriber;
struct IoxFramePublisher;

/**
 * Camera pawn driven by iceoryx2 pose data from sim_metaverse.
 *
 * Subscribes to behave/SimStatus for pose updates.
 * Captures the camera view at 640x480 and publishes raw BGRA frames
 * on behave/video/CameraRawFrame via iceoryx2.
 */
UCLASS()
class BEHAVESIM_API ASimCameraPawn : public APawn
{
	GENERATED_BODY()

public:
	ASimCameraPawn();

	virtual void BeginPlay() override;
	virtual void Tick(float DeltaTime) override;
	virtual void EndPlay(const EEndPlayReason::Type EndPlayReason) override;

private:
	UPROPERTY(VisibleAnywhere, Category = "Camera")
	class UCameraComponent* CameraComp;

	UPROPERTY(VisibleAnywhere, Category = "Capture")
	class USceneCaptureComponent2D* SceneCapture;

	UPROPERTY()
	class UTextureRenderTarget2D* RenderTarget;

	FIoxSimSubscriber* IoxSub = nullptr;
	IoxFramePublisher* FramePub = nullptr;
};
