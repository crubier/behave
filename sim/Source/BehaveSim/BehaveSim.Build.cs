using System.IO;
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
			"InputCore"
		});

		// ── sim_bridge Rust cdylib ────────────────────────────────
		// Build it first:  cd sim/sim_bridge && cargo build --release
		string BridgeDir = Path.GetFullPath(
			Path.Combine(ModuleDirectory, "..", "..", "sim_bridge", "target", "release"));

		if (Target.Platform == UnrealTargetPlatform.Mac)
		{
			string DylibPath = Path.Combine(BridgeDir, "libsim_bridge.dylib");
			PublicAdditionalLibraries.Add(DylibPath);
			PublicDelayLoadDLLs.Add(DylibPath);
			RuntimeDependencies.Add(DylibPath);
		}
		else if (Target.Platform == UnrealTargetPlatform.Linux)
		{
			string SoPath = Path.Combine(BridgeDir, "libsim_bridge.so");
			PublicAdditionalLibraries.Add(SoPath);
			PublicRuntimeLibraryPaths.Add(BridgeDir);
			RuntimeDependencies.Add(SoPath);
		}
		else if (Target.Platform == UnrealTargetPlatform.Win64)
		{
			PublicAdditionalLibraries.Add(Path.Combine(BridgeDir, "sim_bridge.lib"));
			RuntimeDependencies.Add(Path.Combine(BridgeDir, "sim_bridge.dll"));
			PublicDelayLoadDLLs.Add("sim_bridge.dll");
		}
	}
}
