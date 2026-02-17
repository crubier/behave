#include "BehaveSimGameMode.h"
#include "SimCameraPawn.h"

ABehaveSimGameMode::ABehaveSimGameMode()
{
	DefaultPawnClass = ASimCameraPawn::StaticClass();
}

void ABehaveSimGameMode::BeginPlay()
{
	Super::BeginPlay();

	// Show the mouse cursor over the viewport (render-only window).
	if (APlayerController* PC = GetWorld()->GetFirstPlayerController())
	{
		PC->bShowMouseCursor = true;
	}
}
