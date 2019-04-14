using System.Collections.Immutable;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;
using Xunit;

namespace IncrementalSociety.Tests
{
    public class ResourceEngineTests
    {
		[Fact]
		public void AdditionalResourceNextTick ()
		{

		}

		[Fact]
		public void BuildingResources ()
		{
			ResourceEngine engine = Engine ();
			var campResources = engine.GetBuildingResources ("Gathering Camp");
			Assert.True (campResources["Food"] > 0.0);
			Assert.True (campResources["Water"] > 0.0);
			Assert.True (campResources["Stone"] > 0.0);
			Assert.True (campResources["Wood"] > 0.0);
		}
	
		static ResourceEngine Engine ()
		{
			var resources = JsonLoader.Load ();
			ResourceEngine engine = new ResourceEngine (resources);
			return engine;
		}

		[Fact]
        public void AddTwoResourcesDifferentItems ()
        {
			var result = Immutable.CreateBuilderDictionary ("A", 1.0);
			ResourceEngine.AddResources (result, Immutable.CreateDictionary ("B", 1.0));
            Assert.Equal (1, result["A"]);
            Assert.Equal (1, result["B"]);
        }

        [Fact]
        public void AddTwoResourcesWithSameItems ()
        {
			var result = Immutable.CreateBuilderDictionary ("A", 1.0);
			ResourceEngine.AddResources (result, Immutable.CreateDictionary ("A", 1.0));
			Assert.Equal (2, result["A"]);
        }

        [Fact]
        public void AddTwoResourceOneEmpty ()
        {
			var result = Immutable.CreateBuilderDictionary ("A", 1.0);
			ResourceEngine.AddResources (result, ImmutableDictionary<string, double>.Empty);
            Assert.Equal (1, result["A"]);
        }
    }
}
