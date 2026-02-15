using UnrealBuildTool;

public class BehaveSim : ModuleRules
{
	public BehaveSim(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;

		PublicDependencyModuleNames.AddRange(new string[]
		{
			"Core",
			"CoreUObject",
			"Engine",
			"InputCore",
			"Networking",
			"Sockets"
		});
	}
}
