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
			GameState state = Factories.CreateGameState (camps: 1);
			var resources = engine.CalculateAdditionalNextTick (state);
			Assert.True (resources["Food"] > 0.0);
			Assert.True (resources["Water"] > 0.0);
		}
		
		[Fact]
		public void AdditionalResourceNextTickWithConversions ()
		{
			ResourceEngine engine = Factories.CreateResourceEngine ();
			GameState state = Factories.CreateGameState (camps: 1, workshops: 1);
			var resources = engine.CalculateAdditionalNextTick (state);
			Assert.True (resources["Food"] > 0.0);
			Assert.True (resources["Water"] > 0.0);
			Assert.True (resources["Wood"] < 0.0);
			Assert.True (resources["Charcoal"] > 0.0);
		}
		
		[Fact]
		public void AdditionalResourceNextTickWithConversionsDisabled ()
		{
			ResourceEngine engine = Factories.CreateResourceEngine ();
			GameState state = Factories.CreateGameState (camps: 0, workshops: 1).WithDisabledConversions (new string [] {"Conversion"});
			var resources = engine.CalculateAdditionalNextTick (state);
			Assert.Empty (resources);
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
			var conversions = engine.GetBuildingConvertedResources ("Workshop");
			Assert.Equal ("Conversion", conversions[0].Name);
			Assert.True (conversions[0].Resources["Wood"] < 0.0);
			Assert.True (conversions[0].Resources["Charcoal"] > 0.0);
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
