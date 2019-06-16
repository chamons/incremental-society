using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using Xunit;

namespace IncrementalSociety.Tests
{
	public class RegionGeneratorTests : ResourceTestBase
	{
		public RegionGeneratorTests ()
		{
			const string AreaJson = @",{
				""name"": ""Super Plains"",
				""bonus_yield"": [
					{ ""name"": ""Food"", ""amount"" : 2 }
				],
			}";
			const string FeatureJson = "";
			const string ClimateJson = "";
			ConfigureCustomJsonPayload (extraAreaJSON: AreaJson, extraFeatureJSON: FeatureJson, extraClimateJSON: ClimateJson);
		}

		[Fact]
		public void GeneratesAreasBasedOnClimate ()
		{

		}

		[Fact]
		public void AreasHaveFeatures ()
		{

		}

		[Fact]
		public void RegionHasMultipleAreas ()
		{

		}


		[Fact]
		public void RegionHasName ()
		{

		}
	}
}