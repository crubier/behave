using UnrealBuildTool;

public class BehaveSimTarget : TargetRules
{
	public BehaveSimTarget(TargetInfo Target) : base(Target)
	{
		Type = TargetType.Game;
		DefaultBuildSettings = BuildSettingsVersion.Latest;
		IncludeOrderVersion = EngineIncludeOrderVersion.Latest;
		ExtraModuleNames.Add("BehaveSim");
	}
}
