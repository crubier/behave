#include "BehaveSimGameMode.h"
#include "SimCameraPawn.h"
#include "Engine/GameEngine.h"
#include "GameFramework/GameUserSettings.h"
#include "Kismet/KismetSystemLibrary.h"

ABehaveSimGameMode::ABehaveSimGameMode()
{
	DefaultPawnClass = ASimCameraPawn::StaticClass();
}

void ABehaveSimGameMode::BeginPlay()
{
	Super::BeginPlay();

	// Cap rendering to 30 FPS
	if (GEngine)
	{
		GEngine->Exec(GetWorld(), TEXT("t.MaxFPS 30"));
	}

	// Set resolution for standalone/packaged builds
	UGameUserSettings* Settings = GEngine ? GEngine->GetGameUserSettings() : nullptr;
	if (Settings)
	{
		Settings->SetScreenResolution(FIntPoint(1920, 1080));
		Settings->SetFullscreenMode(EWindowMode::Windowed);
		Settings->SetVSyncEnabled(false);
		Settings->ApplySettings(false);
	}

	UE_LOG(LogTemp, Log, TEXT("[BehaveSim] Rendering at 30 FPS, resolution 1920x1080"));
}
