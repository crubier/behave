#pragma once

#include "CoreMinimal.h"
#include "GameFramework/GameModeBase.h"
#include "BehaveSimGameMode.generated.h"

/**
 * Default game mode for BehaveSim.
 *
 * Spawns ASimCameraPawn as the default pawn so the viewport is driven
 * by iceoryx2 pose commands out of the box.
 */
UCLASS()
class BEHAVESIM_API ABehaveSimGameMode : public AGameModeBase
{
	GENERATED_BODY()

public:
	ABehaveSimGameMode();
	virtual void BeginPlay() override;
};
