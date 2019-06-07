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
	public class GrowthRateCurveTests : ResourceTestBase
	{
		[Fact]
		public void PopulationGrowthFactors ()
		{
			var curve = CreatePopulationGrowthCurve ();

			// By default we grow at 1% a tick if fully happy and linerally less as we approach 0% happiness
			// Births don't hit 0% when people are unhappy as not all things are up to planning
			Assert.Equal (1, curve.CalculatePopulationGrowthRate (100, PopulationRatio.Create (1.0)));
			Assert.Equal (.6, curve.CalculatePopulationGrowthRate (100, PopulationRatio.Create (0.5)), 3);
			Assert.Equal (.20, curve.CalculatePopulationGrowthRate (100, PopulationRatio.Create (0)), 3);
		}

		[Fact]
		public void PopulationDeathFactors ()
		{
			var curve = CreatePopulationGrowthCurve ();

			// By default we die at .5% a tick if fully helath and linerally triple that as we approach 0% health
			Assert.Equal (.5, curve.CalculatePopulationDeathRate (100, PopulationRatio.Create (1.0)), 3);
			Assert.Equal (1, curve.CalculatePopulationDeathRate (100, PopulationRatio.Create (0.5)), 3);
			Assert.Equal (1.5, curve.CalculatePopulationDeathRate (100, PopulationRatio.Create (0)), 3);
		}

		[Fact]
		public void PopulationImmigrationFactors ()
		{
			var curve = CreatePopulationGrowthCurve ();

			// When happy (> .5 happines) we immigrate up 1% of open housing per tick.
			// This linerally decreases as we approach .5
			Assert.Equal (1, curve.CalculateImmigrationRate (100, PopulationRatio.Create (1.0)), 3);
			Assert.Equal (.5, curve.CalculateImmigrationRate (100, PopulationRatio.Create (0.75)), 3);
			Assert.Equal (0, curve.CalculateImmigrationRate (100, PopulationRatio.Create (.5)), 3);
			Assert.Equal (0, curve.CalculateImmigrationRate (100, PopulationRatio.Create (0)), 3);
		}

		[Fact]
		public void PopulationEmmigrationFactors ()
		{
			var curve = CreatePopulationGrowthCurve ();

			// When unhappy (< .5 happines) we emmigrate up 1% of population per tick.
			// This linerally decreases as we approach .5
			Assert.Equal (1, curve.CalculateEmmigrationRate (100, PopulationRatio.Create (0), 0), 3);
			Assert.Equal (.5, curve.CalculateEmmigrationRate (100, PopulationRatio.Create (0.25), 0), 3);
			Assert.Equal (0, curve.CalculateEmmigrationRate (100, PopulationRatio.Create (.5), 0), 3);
			Assert.Equal (0, curve.CalculateEmmigrationRate (100, PopulationRatio.Create (1.0), 0), 3);

			// When we are out of room, we emmigrarte 2% of the housing delta per tick
			Assert.Equal (2, curve.CalculateEmmigrationRate (100, PopulationRatio.Create (1.0), -100), 3);
			Assert.Equal (.2, curve.CalculateEmmigrationRate (100, PopulationRatio.Create (1.0), -10), 3);
		}


		[Fact]
		public void RoundGrowthRateToPreventOverflows ()
		{
			var curve = CreatePopulationGrowthCurve ();

			// No rounding when not going over
			Assert.Equal (.5, curve.RoundGrowthToPreventOverflow (299.1, .5, 300));
			Assert.Equal (-.5, curve.RoundGrowthToPreventOverflow (299.1, -.5, 300));

			// Rounding when going over
			Assert.Equal (.9, curve.RoundGrowthToPreventOverflow (299.1, 1.2, 300), 3);
			Assert.Equal (.1, curve.RoundGrowthToPreventOverflow (299.9, 1.2, 300), 3);

			// Rounding when going under
			Assert.Equal (-.9, curve.RoundGrowthToPreventOverflow (100.9, -1.2, 300), 3);
			Assert.Equal (-.1, curve.RoundGrowthToPreventOverflow (100.1, -1.2, 300), 3);
		}

		[Fact]
		public void RoundGrowthWithNoCapShouldHitZero ()
		{
			var curve = CreatePopulationGrowthCurve ();

			Assert.Equal (0, curve.RoundGrowthToPreventOverflow (100, -.5, 0));
			Assert.Equal (0, curve.RoundGrowthToPreventOverflow (100, .5, 0));
		}

		[Fact]
		public void RoundGrowthAtCapShouldHitZero ()
		{
			var curve = CreatePopulationGrowthCurve ();

			Assert.Equal (0, curve.RoundGrowthToPreventOverflow (100, -.5, 100));
			Assert.Equal (0, curve.RoundGrowthToPreventOverflow (200, .5, 200));
		}

		[Fact]
		public void RoundGrowthToPreventVerySmallGrowths ()
		{
			var curve = CreatePopulationGrowthCurve ();

			Assert.Equal (1.2, curve.RoundGrowthRateAboveMinimumStep (1.2));
			Assert.Equal (curve.MinGrowth, curve.RoundGrowthRateAboveMinimumStep (0.01));

			Assert.Equal (-1.2, curve.RoundGrowthRateAboveMinimumStep (-1.2));
			Assert.Equal (-curve.MinGrowth, curve.RoundGrowthRateAboveMinimumStep (-0.01));
		}

		[Fact]
		public void DroppingHousingDoesNotDrasticallyReducePopulation ()
		{
			var curve = CreatePopulationGrowthCurve ();
			var state = CreateGameState ();
			var buildingEngine = CreateBuildingEngine ();

			var growthRate = curve.GetGrowthRate (state, PopulationRatio.Create (1),PopulationRatio.Create (1));
			Assert.True (growthRate < 0 && growthRate > -10);
		}

		[Fact]
		public void RoundGrowthDoesNotChangeSign ()
		{
			var curve = CreatePopulationGrowthCurve ();
			var growthRate = curve.RoundGrowthToPreventOverflow (200, 1, 0);
			Assert.Equal (0, growthRate);
		}
	}
}
