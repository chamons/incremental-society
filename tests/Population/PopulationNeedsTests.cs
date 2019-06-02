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
			Assert.Equal (1, needs.CalculateHappiness (200, new double [] { }, false).Value, 3);
			Assert.Equal (.8, needs.CalculateHappiness (200, new double [] { 0 }, false).Value, 3);
			Assert.Equal (.6, needs.CalculateHappiness (200, new double [] { 0, 0 }, false).Value, 3);

			// Each complete luxary good needs is 10% bonus
			Assert.Equal (1, needs.CalculateHappiness (200, new double [] { 1 }, false).Value, 3);

			// And they can cancel out somewhat
			Assert.Equal (.95, needs.CalculateHappiness (200, new double [] { 1, .75, .75, .75 }, false).Value, 3);
		}

		[Fact]
		public void CrowdingCausesNegativeHappiness ()
		{
			// Population density causes base unhappiness as it increases
			// Each 200 people over 1000 cause 10% and will need to be offset by happiness buildings
			var needs = CreatePopulationNeeds ();
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
		}
	}
}
