using IncrementalSociety.Model;

namespace IncrementalSociety
{
    public class GameEngine
    {
        public static GameState CreateNewGame ()
        {
            var greenlandRegion = new Region ("Greenland", new Area[] { new Area (AreaType.Forest), new Area (AreaType.Plains), new Area (AreaType.Forest), new Area (AreaType.Forest) , new Area (AreaType.Ocean) });
            var mudFlatsRegion = new Region ("Mudflats", new Area[] { new Area (AreaType.Swamp), new Area (AreaType.Swamp), new Area (AreaType.Swamp), new Area (AreaType.Plains), new Area (AreaType.Desert) });
            var resources = new Resource[] { new Resource ("Food", 100), new Resource ("Wood", 50) };
            return new GameState (Age.Stone, new Region[] { greenlandRegion, mudFlatsRegion }, resources);
        }
    }
}
