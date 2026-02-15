#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Pawn.h"
#include "SimCameraPawn.generated.h"

struct FSimPosePacket;

/**
 * Camera pawn driven by UDP pose packets from sim_metaverse.
 *
 * Listens on UDP port 9876 for 64-byte FSimPosePacket structs
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

	class FSocket* UdpSocket = nullptr;
};
