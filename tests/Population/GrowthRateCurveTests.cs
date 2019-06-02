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
			double lowRate = curve.GetGrowthRate (100, 200);
			double mideRate = curve.GetGrowthRate (150, 200);
			double highRate = curve.GetGrowthRate (190, 200);
			Assert.True (lowRate > mideRate && mideRate > highRate);

			double lowOverRate = curve.GetGrowthRate (200, 100);
			double mideOverRate = curve.GetGrowthRate (150, 100);
			double highOverRate = curve.GetGrowthRate (110, 100);
			Assert.True (lowOverRate < mideOverRate && mideOverRate < highOverRate);
		}
	}
}
