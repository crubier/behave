#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Pawn.h"
#include "SimCameraPawn.generated.h"

/**
 * Camera pawn driven by iceoryx2 pose commands.
 *
 * Every tick the pawn polls the sim_bridge for a new camera pose and
 * teleports itself (and its camera) to that location/orientation.
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

	bool bBridgeInitialized = false;
};
