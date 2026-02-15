#include "BehaveSimGameMode.h"
#include "SimCameraPawn.h"

ABehaveSimGameMode::ABehaveSimGameMode()
{
	DefaultPawnClass = ASimCameraPawn::StaticClass();
}
