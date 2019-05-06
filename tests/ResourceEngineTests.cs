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
			ResourceEngine engine = Factories.CreateResourceEngine ();
			GameState state = Factories.CreateGameStateWithOneCamp ();
			var resources = engine.CalculateAdditionalNextTick (state);
			Assert.True (resources["Food"] > 0.0);
			Assert.True (resources["Water"] > 0.0);
		}

		[Fact]
		public void BuildingResources ()
		{
			ResourceEngine engine = Factories.CreateResourceEngine ();
			var campResources = engine.GetBuildingResources ("Gathering Camp");
			Assert.True (campResources["Food"] > 0.0);
			Assert.True (campResources["Water"] > 0.0);
		}
	
		[Fact]
		public void ConversionYield ()
		{
			ResourceEngine engine = Factories.CreateResourceEngine ();
			var campResources = engine.GetBuildingResources ("Workshop");
			Assert.True (campResources["Wood"] < 0.0);
			Assert.True (campResources["Charcoal"] > 0.0);
		}

		[Fact]
		public void AddTwoResourcesDifferentItems ()
		{
			var result = Immutable.CreateBuilderDictionary ("Food", 1.0);
			ResourceEngine.AddResources (result, Immutable.CreateDictionary ("Water", 1.0));
			Assert.Equal (1, result["Food"]);
			Assert.Equal (1, result["Water"]);
		}

		[Fact]
		public void AddTwoResourcesWithSameItems ()
		{
			var result = Immutable.CreateBuilderDictionary ("Food", 1.0);
			ResourceEngine.AddResources (result, Immutable.CreateDictionary ("Food", 1.0));
			Assert.Equal (2, result["Food"]);
		}

		[Fact]
		public void AddTwoResourceOneEmpty ()
		{
			var result = Immutable.CreateBuilderDictionary ("Food", 1.0);
			ResourceEngine.AddResources (result, ImmutableDictionary<string, double>.Empty);
			Assert.Equal (1, result["Food"]);
		}
	}
}
