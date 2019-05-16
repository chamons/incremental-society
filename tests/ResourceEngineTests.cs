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
		public void AddTickOfResources ()
		{
			ResourceEngine engine = Factories.CreateResourceEngine ();
			GameState state = Factories.CreateGameState (camps: 1);
			state = engine.AddTickOfResources (state, 1.0);
			Assert.True (state.Resources["Food"] > 0.0);
			Assert.True (state.Resources["Water"] > 0.0);
		}
		
		[Fact]
		public void AddTickOfResourcesWithInvalidConversion ()
		{
			ResourceEngine engine = Factories.CreateResourceEngine ();
			GameState state = Factories.CreateGameState (workshops: 1);
			state = engine.AddTickOfResources (state, 1.0);
			Assert.False (state.Resources.ContainsKey ("Charcoal"));
			Assert.Contains (state.DisabledConversions, x => x == "Conversion");
		}

		[Fact]
		public void DisablesOnlyOneConversionWhenShort ()
		{
			ResourceEngine engine = Factories.CreateResourceEngine ();
			GameState state = Factories.CreateGameState (workshops: 1, smokers: 1);
			state = state.WithResources (Immutable.CreateBuilderDictionary ("Charcoal", 10.0));
			state = engine.AddTickOfResources (state, 1.0);
			Assert.True (state.Resources["Food"] > 0.0);
			Assert.Single (state.DisabledConversions);
			Assert.Contains (state.DisabledConversions, x => x == "Conversion");
		}

		[Fact]
		public void AdditionalResourceNextTick ()
		{
			ResourceEngine engine = Factories.CreateResourceEngine ();
			GameState state = Factories.CreateGameState (camps: 1);
			var resources = engine.CalculateAdditionalNextTick (state, 1.0);
			Assert.True (resources["Food"] > 0.0);
			Assert.True (resources["Water"] > 0.0);
		}

		[Fact]
		public void AdditionalResourceNextTickWithEfficiency ()
		{
			ResourceEngine engine = Factories.CreateResourceEngine ();
			GameState state = Factories.CreateGameState (camps: 1);
			var baseResources = engine.CalculateAdditionalNextTick (state, 1.0);
			var extraResources = engine.CalculateAdditionalNextTick (state, 1.1);
			var lessResources = engine.CalculateAdditionalNextTick (state, .9);
			Assert.True (baseResources.HasMoreThan (lessResources));
			Assert.True (extraResources.HasMoreThan (baseResources));
		}

		[Fact]
		public void AdditionalResourceNextTickWithConversions ()
		{
			ResourceEngine engine = Factories.CreateResourceEngine ();
			GameState state = Factories.CreateGameState (camps: 1, workshops: 1);
			var resources = engine.CalculateAdditionalNextTick (state, 1.0);
			Assert.True (resources["Food"] > 0.0);
			Assert.True (resources["Water"] > 0.0);
			Assert.True (resources["Wood"] < 0.0);
			Assert.True (resources["Charcoal"] > 0.0);
		}

		[Fact]
		public void AdditionalResourceNextTickWithConversionsAndEfficiency ()
		{
			ResourceEngine engine = Factories.CreateResourceEngine ();
			GameState state = Factories.CreateGameState (workshops: 1);
			var baseResources = engine.CalculateAdditionalNextTick (state, 1.0);
			var lessResources = engine.CalculateAdditionalNextTick (state, .9);
			var extraResources = engine.CalculateAdditionalNextTick (state, 1.1);

			// Conversions should ignore efficiency
			Assert.Equal (baseResources.AmountOf("Charcoal"), lessResources.AmountOf ("Charcoal"));
			Assert.Equal (baseResources.AmountOf("Wood"), lessResources.AmountOf ("Wood"));
			Assert.Equal (extraResources.AmountOf ("Charcoal"), baseResources.AmountOf ("Charcoal"));
			Assert.Equal (extraResources.AmountOf ("Wood"), baseResources.AmountOf ("Wood"));
		}

		[Fact]
		public void AdditionalResourceNextTickWithConversionsDisabled ()
		{
			ResourceEngine engine = Factories.CreateResourceEngine ();
			GameState state = Factories.CreateGameState (camps: 0, workshops: 1).WithDisabledConversions ("Conversion".Yield ());
			var resources = engine.CalculateAdditionalNextTick (state, 1.0);
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
			var conversions = engine.GetBuildingConversionResources ("Workshop");
			Assert.Equal ("Conversion", conversions[0].Name);
			Assert.True (conversions[0].Resources["Wood"] < 0.0);
			Assert.True (conversions[0].Resources["Charcoal"] > 0.0);
		}
		
		[Fact]
		public void ReturnsEnabledConversions ()
		{
			GameState state = Factories.CreateGameState (workshops: 1);
			ResourceEngine engine = Factories.CreateResourceEngine ();
			var conversions = engine.GetConversions (state);
			Assert.Single (conversions);
			Assert.Contains (conversions, x => x.Name == "Conversion" && x.Enabled);
		}

		[Fact]
		public void ReturnsSingleConversionWhenMultipleBuildings ()
		{
			GameState state = Factories.CreateGameState (workshops: 2);
			ResourceEngine engine = Factories.CreateResourceEngine ();
			var conversions = engine.GetConversions (state);
			Assert.Single (conversions);
		}
	}
}
