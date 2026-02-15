using UnrealBuildTool;

public class BehaveSimEditorTarget : TargetRules
{
	public BehaveSimEditorTarget(TargetInfo Target) : base(Target)
	{
		Type = TargetType.Editor;
		DefaultBuildSettings = BuildSettingsVersion.V5;
		IncludeOrderVersion = EngineIncludeOrderVersion.Latest;
		ExtraModuleNames.Add("BehaveSim");
	}
}
