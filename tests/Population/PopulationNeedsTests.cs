using System;
using System.IO;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using IncrementalSociety.Population;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

using Newtonsoft.Json;
using Xunit;

namespace IncrementalSociety.Tests.Population
{
	// See ResourceTestBase xml for values referred to here
	public class PopulationNeedsTests : ResourceTestBase
	{
		[Fact]
		public void PopulationNeedsLuxuryGoodsForHappiness ()
		{
			// Each missing luxury good is up to 20% unhappiness
			var needs = CreatePopulationNeeds ();
			Assert.Equal (1, needs.CalculateHappiness (200, new double [] { }, false).Value, 3);
			Assert.Equal (.8, needs.CalculateHappiness (200, new double [] { 0 }, false).Value, 3);
			Assert.Equal (.6, needs.CalculateHappiness (200, new double [] { 0, 0 }, false).Value, 3);

			// Each complete luxury good needs is 10% bonus
			Assert.Equal (1, needs.CalculateHappiness (200, new double [] { 1 }, false).Value, 3);

			// And they can cancel out somewhat
			Assert.Equal (.95, needs.CalculateHappiness (200, new double [] { 1, .75, .75, .75 }, false).Value, 3);
		}

		[Fact]
		public void PopulationLuxuryFromActualBulidings ()
		{
			GameState state = CreateGameState ();
			state = state.WithResources (Create ("Water", 100));
			BuildingEngine buildingEngine = CreateBuildingEngine ();

			var needs = CreatePopulationNeeds ();
			Assert.Equal (.8, needs.CalculateHappiness (state).Value, 3);

			state = buildingEngine.Build (state, state.Regions[0].Name, 0, "Potter");
			double happiness = needs.CalculateHappiness (state).Value;
			Assert.True (happiness > .8 && happiness < 1);
		}

		[Fact]
		public void CrowdingCausesNegativeHappiness ()
		{
			// Population density causes base unhappiness as it increases
			// Each 200 people over 1000 cause 10% and will need to be offset by happiness buildings
			var needs = CreatePopulationNeeds ();
			Assert.Equal (1, needs.CalculateHappiness (800, Enumerable.Empty<double> (), false).Value, 3);
			Assert.Equal (1, needs.CalculateHappiness (1000, Enumerable.Empty<double> (), false).Value, 3);
			Assert.Equal (.9, needs.CalculateHappiness (1200, Enumerable.Empty<double> (), false).Value, 3);
			Assert.Equal (.6, needs.CalculateHappiness (1800, Enumerable.Empty<double> (), false).Value, 3);
		}

		[Fact]
		public void StarvingSetHappinessToZero ()
		{
			var needs = CreatePopulationNeeds ();
			Assert.Equal (0, needs.CalculateHappiness (200, new double [] { 1 }, true).Value);
		}

		[Fact]
		public void CrowdingCausesNegativeHealth ()
		{
			// Population density causes base unhappiness as it increases
			// Each 200 people over 1000 cause 10% and will need to be offset by health buildings
			var needs = CreatePopulationNeeds ();
			Assert.Equal (1, needs.CalculateHealth (800).Value, 3);
			Assert.Equal (1, needs.CalculateHealth (1000).Value, 3);
			Assert.Equal (.9, needs.CalculateHealth (1200).Value, 3);
			Assert.Equal (.6, needs.CalculateHealth (1800).Value, 3);
		}
	}
}
