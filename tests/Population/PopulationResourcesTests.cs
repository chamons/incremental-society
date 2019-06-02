using System;
using System.IO;
using System.Collections.Generic;
using System.Collections.Immutable;
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
		}
	}
}
