using System;
using System.IO;
using System.Collections.Generic;
using System.Collections.Immutable;
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
		public void PopulationNeedsLuxaryGoodsForHappiness ()
		{
			// Each missing luxary good is up to 20% unhappiness
			var needs = CreatePopulationNeeds ();
			GameState state = CreateGameState ();
			Assert.Equal (1, needs.CalculateHappiness (state).Value);
		}

		[Fact]
		public void CrowdingCausesNegativeHappiness ()
		{
			// Population density causes base unhappiness as it increases
			// Each 200 people over 1000 cause 10% and will need to be offset by happiness buildings
			var needs = CreatePopulationNeeds ();
			GameState state = CreateGameState ();
			Assert.Equal (1, needs.CalculateHappiness (state).Value);
		}

		[Fact]
		public void CrowdingCausesNegativeHealth ()
		{
			// Population density causes base unhappiness as it increases
			// Each 200 people over 1000 cause 10% and will need to be offset by health buildings
			var needs = CreatePopulationNeeds ();
			GameState state = CreateGameState ();
			Assert.Equal (1, needs.CalculateHealth (state).Value);
		}
	}
}
