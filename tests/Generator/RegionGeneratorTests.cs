using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

using Xunit;

using IncrementalSociety.Tests;
using IncrementalSociety.Utilities;

namespace IncrementalSociety.Generator.Tests
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
			const string ClimateJson = @"
			{
				""name"": ""TestClimate"",
				""area_chances"": [
					{ ""name"": ""First"", ""chance"": 0.2 },
					{ ""name"": ""Second"", ""chance"": 0.8 },
					{ ""name"": ""Third"", ""chance"": 0 },
				],
				""feature_chance"": 1
			},
			{
				""name"": ""OtherTestClimate"",
				""area_chances"": [
					{ ""name"": ""First"", ""chance"": 0 },
					{ ""name"": ""Second"", ""chance"": 0.2 },
					{ ""name"": ""Third"", ""chance"": 0.8 },
				],
				""feature_chance"": 0
			}
			";
			ConfigureCustomJsonPayload (extraAreaJSON: AreaJson, extraFeatureJSON: FeatureJson, extraClimateJSON: ClimateJson);
		}

		[Fact]
		public void GeneratesAreasBasedOnClimate ()
		{
			var regionGenerator = CreateRegionGenerator ();
			var firstClimate = 500.Range().Select (x => regionGenerator.CreateArea ("TestClimate")).ToList ();
			var secondClimate = 500.Range ().Select (x => regionGenerator.CreateArea ("OtherTestClimate")).ToList ();

			// Technically this is a possible random failure, but if .8 doesn't hit over 500 tries we are _very_ unlucky
			Assert.True (firstClimate.Count (x => x.Type == "First") > 0);
			Assert.True (firstClimate.Count (x => x.Type == "Third") == 0);
			Assert.True (firstClimate.Count (x => x.Type == "Second") > secondClimate.Count (x => x.Type == "Second"));
			Assert.True (secondClimate.Count (x => x.Type == "First") == 0);
			Assert.True (secondClimate.Count (x => x.Type == "Third") > 0);
		}

		[Fact]
		public void AreasHaveFeatures ()
		{
			var regionGenerator = CreateRegionGenerator ();

		}

		[Fact]
		public void RegionHasMultipleAreas ()
		{
			var regionGenerator = CreateRegionGenerator ();
			Assert.True (regionGenerator.CreateRegion (RegionSize.Small, "TestClimate").Areas.Count () > 2);
			Assert.True (regionGenerator.CreateRegion (RegionSize.Medium, "TestClimate").Areas.Count () > 3);
			Assert.True (regionGenerator.CreateRegion (RegionSize.Large, "TestClimate").Areas.Count () > 5);
		}

		[Fact]
		public void RegionHasName ()
		{
			var regionGenerator = CreateRegionGenerator ();
			Assert.Equal ("TestLand", regionGenerator.CreateRegion (RegionSize.Medium, "TestClimate").Name);
		}
	}
}