namespace IncrementalSociety.Model
{
	// This is a template for https://github.com/chamons/VinylCutter
	// Run dotnet records GameState.template to update GameState.g.cs 
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

	public class Area
	{
		AreaType Type;

		[Default ("null")]
		List<string> Buildings;
	}

	public class Region
	{
		string Name;
		List<Area> Areas;
	}

	[With]
	public class GameState
	{
		Age Age;
		List<Region> Regions;
		Dictionary<string, double> Resources;
	}
}
