using UnrealBuildTool;

public class BehaveSimEditorTarget : TargetRules
{
	public BehaveSimEditorTarget(TargetInfo Target) : base(Target)
	{
		Type = TargetType.Editor;
		DefaultBuildSettings = BuildSettingsVersion.Latest;
		IncludeOrderVersion = EngineIncludeOrderVersion.Latest;
		ExtraModuleNames.Add("BehaveSim");
	}
}
