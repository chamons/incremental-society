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
		public void PopsGrowRateBasedOnSpaceToCap ()
		{
			var curve = CreatePopulationGrowthCurve ();
			double lowRate = curve.GetBaseGrowthRate (100, 200);
			double mideRate = curve.GetBaseGrowthRate (150, 200);
			double highRate = curve.GetBaseGrowthRate (190, 200);
			Assert.True (lowRate > mideRate && mideRate > highRate);

			double lowOverRate = curve.GetBaseGrowthRate (200, 100);
			double mideOverRate = curve.GetBaseGrowthRate (150, 100);
			double highOverRate = curve.GetBaseGrowthRate (110, 100);
			Assert.True (lowOverRate < mideOverRate && mideOverRate < highOverRate);
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
		public void RoundGrowthToPreventVerySmallGrowths ()
		{
			var curve = CreatePopulationGrowthCurve ();

			Assert.Equal (1.2, curve.RoundGrowthRateAboveMinimumStep (1.2));
			Assert.Equal (PopulationGrowthCurve.MinGrowth, curve.RoundGrowthRateAboveMinimumStep (0.01));

			Assert.Equal (-1.2, curve.RoundGrowthRateAboveMinimumStep (-1.2));
			Assert.Equal (-PopulationGrowthCurve.MinGrowth, curve.RoundGrowthRateAboveMinimumStep (-0.01));
		}
	}
}
