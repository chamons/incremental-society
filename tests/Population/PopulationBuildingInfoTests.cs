using System;
using System.IO;
using System.Collections.Generic;
using System.Collections.Immutable;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

using Newtonsoft.Json;
using Xunit;

namespace IncrementalSociety.Tests.Population
{
	public class PopulationBuildingInfoTests : ResourceTestBase
	{
		[Fact]
		public void PopulationEfficiencyRatios ()
		{
			var popBuildingInfo = CreatePopulationBuildingInfo ();

			double baseValue = popBuildingInfo.GetPopulationEfficiency (4, 5);
			double someOverValue = popBuildingInfo.GetPopulationEfficiency (6, 5);
			double moreOverValue = popBuildingInfo.GetPopulationEfficiency (7, 5);
			Assert.Equal (1.0, baseValue);
			Assert.True (baseValue > someOverValue);
			Assert.True (someOverValue > moreOverValue);
		}

		[Fact]
		public void MoreBuildingsThanPopsReducesEfficiency ()
		{
			var popBuildingInfo = CreatePopulationBuildingInfo ();
			var state = CreateGameState (camps: 1);
			double baseEfficiency = popBuildingInfo.GetPopulationEfficiency (state);
			Assert.Equal (1.0, baseEfficiency);

			state = CreateGameState (camps: 2);
			double lessEfficiency = popBuildingInfo.GetPopulationEfficiency (state);
			Assert.True (lessEfficiency < baseEfficiency);
			Assert.True (lessEfficiency != 0);
		}

		[Fact]
		public void SomeBuildingsDoNotDecreaseEfficiency ()
		{
			const string extraBuildingJSON = @",
			{
				""name"": ""NoJob"",
				""valid_areas"": [""Any""],
				""does_not_require_job"": true
			}";
			ConfigureCustomJsonPayload (extraBuildingJSON: extraBuildingJSON);

			var popBuildingInfo = CreatePopulationBuildingInfo ();
			var state = CreateGameState (camps: 1);
			double baseEfficiency = popBuildingInfo.GetPopulationEfficiency (state);
			Assert.Equal (1.0, baseEfficiency);

			var buildingEngine = CreateBuildingEngine ();
			state = buildingEngine.Build (state, state.Regions[0].Name, 0, "NoJob");

			double afterEfficiency = popBuildingInfo.GetPopulationEfficiency (state);
			Assert.Equal (1.0, afterEfficiency);
		}
	}
}
