#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Pawn.h"
#include "SimCameraPawn.generated.h"

class FIoxSimSubscriber;

/**
 * Camera pawn driven by iceoryx2 pose data from sim_metaverse.
 *
 * Subscribes to the behave/SimStatus topic via shared memory
 * and teleports the camera to the received pose each frame.
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

	FIoxSimSubscriber* IoxSub = nullptr;
};
