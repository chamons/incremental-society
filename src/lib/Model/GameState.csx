namespace IncrementalSociety.Model
{
	// This is a template for https://github.com/chamons/VinylCutter
	// Run dotnet records GameState.cs to update GameState.g.cs
	[Inject]
	public enum AreaType
	{
		Forest,
		Plains,
		Mountain,
		Ocean,
		Swamp,
		Desert
	}

	[Inject]
	public enum Age
	{
		Stone,
		Bronze
	}

	[With]
	public class Area
	{
		AreaType Type;

		[Default ("null")]
		List<string> Buildings;
	}

	[With]
	public class Region
	{
		string Name;
		List<Area> Areas;
	}

	[Skip]
	public class Resources {}

	[Skip]
	public class EdictCooldown {}

	[With]
	public class GameState
	{
		[Inject]
		public bool HasResearch (string tech) => tech == null || ResearchUnlocks.Contains (tech);

		int Version;

		Age Age;

		List<Region> Regions;

		Resources Resources;

		double Population;

		double PopulationCap;

		EdictCooldown Edicts;

		[Default ("null")]
		HashSet<string> ResearchUnlocks;

		[Default ("null")]
		HashSet<string> DisabledConversions;
	}

	public class ResearchItem
	{
		string Name;
		string Description;
		Resources Cost;
	}
}
