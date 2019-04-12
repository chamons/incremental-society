using IncrementalSociety.Json;
using Xunit;

namespace IncrementalSociety.Tests
{
    public class JsonLoaderTest
    {
        [Fact]
        public void FindsExpectedResourceBundles ()
        {
            var resources = JsonLoader.Load ();
            Assert.NotNull (resources.ActionsJSON);
            Assert.NotNull (resources.BuildingsJSON);
            Assert.NotNull (resources.GameJSON);
            Assert.NotNull (resources.RegionsJSON);
            Assert.NotNull (resources.ResourcesJSON);
        }

        [Fact]
        public void SmokeTest ()
        {
            var resources = JsonLoader.Load ();
            Assert.Contains (resources.Resources.Resources, x => x.Name == "Food");

            Assert.Contains (resources.Regions.Regions, x => x.Name == "Forest");

            Assert.Contains (resources.Game.Ages, x => x == "Stone");
            Assert.Contains (resources.Game.Population_needs, x => x.Resource == "Food");

            Assert.Contains (resources.Buildings.Buildings, x => x.Name == "Gathering Camp");
            Assert.Contains (resources.Buildings.Settlements, x => x.Name == "Crude Settlement");

            Assert.Contains (resources.Buildings.Buildings, x => x.Name == "Gathering Camp");
            Assert.Contains (resources.Buildings.Settlements, x => x.Name == "Crude Settlement");

            Assert.Contains (resources.Actions.Actions, x => x.Name == "Grow Population");
        }
    }
}
