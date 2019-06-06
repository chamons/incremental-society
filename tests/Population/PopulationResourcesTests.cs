using System;
using System.IO;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

using Newtonsoft.Json;
using Xunit;

namespace IncrementalSociety.Tests.Population
{
	public class PopulationResourcesTests : ResourceTestBase
	{
		[Fact]
		public void GetRequiredResourcesForPop ()
		{
			var resourceFinder = CreatePopulationResources ();
			var state = CreateGameState ();
			state = state.WithPopulation (100);

			var reqs = resourceFinder.GetRequirementsForCurrentPopulation (state);
			Assert.Equal (1, reqs["Water"]);
			state = state.WithPopulation (200);
			reqs = resourceFinder.GetRequirementsForCurrentPopulation (state);
			Assert.Equal (2, reqs["Water"]);
		}

		[Fact]
		public void PopulationRequirementsMayChangeDueToTechnology ()
		{
			var resourceFinder = CreatePopulationResources ();
			var state = CreateGameState ();
			var reqs = resourceFinder.GetRequirementsForCurrentPopulation (state);
			Assert.True (reqs["Water"] > 0);
			Assert.True (reqs["Food"] == 0);

			state = state.WithResearchUnlocks (new string [] { "Bronze Age" });

			reqs = resourceFinder.GetRequirementsForCurrentPopulation (state);
			Assert.True (reqs["Water"] > 0);
			Assert.True (reqs["Food"] > 0);
		}

		[Fact]
		public void PopsStarvingWhenNotEnoughResources ()
		{
			var resourceFinder = CreatePopulationResources ();
			var state = CreateGameState ();
			state = state.WithPopulation (100);

			Assert.True (resourceFinder.IsPopulationStarving (state));

			// Even if we're in the negative, we only starve when nothing stored
			state = state.WithResources (Create ("Water", 100));
			Assert.False (resourceFinder.IsPopulationStarving (state));
		}

		[Fact]
		public void GetLuxaryResourcesForPop ()
		{
			var resourceFinder = CreatePopulationResources ();
			var state = CreateGameState ();
			state = state.WithPopulation (100);

			var reqs = resourceFinder.GetLuxaryForCurrentPopulation (state);
			Assert.Equal (1, reqs["Pottary"]);
			state = state.WithPopulation (200);
			reqs = resourceFinder.GetLuxaryForCurrentPopulation (state);
			Assert.Equal (2, reqs["Pottary"]);
		}

		[Fact]
		public void LuxaryResourceRatios ()
		{
			GameState state = CreateGameState ();
			BuildingEngine buildingEngine = CreateBuildingEngine ();

			var resourceFinder = CreatePopulationResources ();
			var ratios = resourceFinder.FindLuxaryRatios (state).ToList ();
			Assert.Single (ratios);
			Assert.Equal(0, ratios[0]);

			state = buildingEngine.Build (state, state.Regions[0].Name, 0, "Potter");

			ratios = resourceFinder.FindLuxaryRatios (state).ToList ();
			Assert.Single (ratios);
			Assert.Equal(0.1, ratios[0], 3);
		}
	}
}
