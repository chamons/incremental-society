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
				""name"": ""First"",
				""name"": ""Second"",
				""name"": ""Third"",
			}";
			const string FeatureJson = @"{
				""name"": ""Fertile"",
				""bonus_yield"": [
					{ ""name"": ""Food"", ""amount"": 1.2 }
				]
			}";
			const string ClimateJson = @"{
				""name"": ""TestClimate"",
				""area_chances"": [
					{ ""name"": ""First"", ""chance"": 0.2 },
					{ ""name"": ""Second"", ""chance"": 0.8 },
					{ ""name"": ""Third"", ""chance"": 0 },
				],
				""feature_chance"": 0.5
			}";
			ConfigureCustomJsonPayload (extraAreaJSON: AreaJson, extraFeatureJSON: FeatureJson, extraClimateJSON: ClimateJson);
		}

		[Fact]
		public void GeneratesAreasBasedOnClimate ()
		{
			var regionRegenerator = CreateRegionGenerator ();
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