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
    }

    public class Region
    {
        string Name;
        List<Area> Areas;
    }

    public class Resource
    {
        string Name;
        int Amount;
    }

    public class GameState
    {
        Age Age;
        List<Region> Regions;
        List<Resource> Resources;
    }
}
