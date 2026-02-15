#include "SimCameraPawn.h"
#include "SimBridge.h"
#include "Camera/CameraComponent.h"
#include "Networking.h"
#include "SocketSubsystem.h"

static constexpr int32 UDP_PORT = 9876;

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

	ISocketSubsystem* SocketSub = ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM);
	if (!SocketSub)
	{
		UE_LOG(LogTemp, Error, TEXT("[BehaveSim] No socket subsystem"));
		return;
	}

	UdpSocket = FUdpSocketBuilder(TEXT("SimMetaverseSocket"))
		.AsNonBlocking()
		.AsReusable()
		.BoundToAddress(FIPv4Address::Any)
		.BoundToPort(UDP_PORT)
		.Build();

	if (UdpSocket)
	{
		int32 BufferSize = 65536;
		UdpSocket->SetReceiveBufferSize(BufferSize, BufferSize);
		UE_LOG(LogTemp, Log, TEXT("[BehaveSim] UDP socket listening on port %d"), UDP_PORT);
	}
	else
	{
		UE_LOG(LogTemp, Error, TEXT("[BehaveSim] Failed to bind UDP socket on port %d"), UDP_PORT);
	}
}

void ASimCameraPawn::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);

	if (!UdpSocket)
	{
		return;
	}

	// Drain all pending UDP packets, keep the latest pose
	FSimPosePacket Pkt;
	bool bGotPose = false;

	uint32 PendingSize = 0;
	while (UdpSocket->HasPendingData(PendingSize))
	{
		uint8 Buffer[128];
		int32 BytesRead = 0;
		TSharedRef<FInternetAddr> Sender = ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->CreateInternetAddr();

		if (UdpSocket->RecvFrom(Buffer, sizeof(Buffer), BytesRead, *Sender))
		{
			if (BytesRead == sizeof(FSimPosePacket))
			{
				FMemory::Memcpy(&Pkt, Buffer, sizeof(FSimPosePacket));
				bGotPose = true;
			}
		}
	}

	if (bGotPose)
	{
		// Schema uses meters; UE5 uses centimetres.
		const FVector Location(Pkt.X * 100.0, Pkt.Y * 100.0, Pkt.Z * 100.0);

		// Quaternion order: UE5 FQuat(X, Y, Z, W)
		const FQuat Rotation(Pkt.QX, Pkt.QY, Pkt.QZ, Pkt.QW);

		SetActorLocationAndRotation(Location, Rotation.IsNormalized() ? Rotation : FQuat::Identity);
	}
}

void ASimCameraPawn::EndPlay(const EEndPlayReason::Type EndPlayReason)
{
	if (UdpSocket)
	{
		UdpSocket->Close();
		ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->DestroySocket(UdpSocket);
		UdpSocket = nullptr;
		UE_LOG(LogTemp, Log, TEXT("[BehaveSim] UDP socket closed"));
	}

	Super::EndPlay(EndPlayReason);
}
