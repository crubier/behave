using UnrealBuildTool;
using System.IO;

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
		});

		// iceoryx2 bridge (pre-built shared library with simple C API)
		string ThirdPartyPath = Path.Combine(ModuleDirectory, "..", "ThirdParty", "iceoryx2");
		string IncludePath = Path.Combine(ThirdPartyPath, "include");
		string LibPath = Path.Combine(ThirdPartyPath, "lib");

		PublicSystemIncludePaths.Add(IncludePath);

		// Link and deploy the bridge dylib + its dependencies
		string[] Dylibs = { "libiceoryx2_bridge.dylib", "libiceoryx_hoofs.2.dylib", "libiceoryx_platform.2.dylib" };
		foreach (string Dylib in Dylibs)
		{
			string FullPath = Path.Combine(LibPath, Dylib);
			PublicAdditionalLibraries.Add(FullPath);
			RuntimeDependencies.Add("$(BinaryOutputDir)/" + Dylib, FullPath);
		}
	}
}
