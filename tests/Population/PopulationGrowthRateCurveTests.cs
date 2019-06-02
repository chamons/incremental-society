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
	}
}
