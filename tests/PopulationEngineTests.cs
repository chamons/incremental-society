using System;
using System.IO;
using System.Collections.Generic;
using System.Collections.Immutable;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

using Newtonsoft.Json;
using Xunit;

namespace IncrementalSociety.Tests
{
	public class PopulationEngineTests : ResourceTestBase
	{
		[Fact]
		public void GetRequiredResourcesForPop ()
		{
			var engine = CreatePopEngine ();
			var state = CreateGameState ();
			state = state.WithPopulation (100);

			var reqs = engine.GetRequirementsForCurrentPopulation (state);
			Assert.Equal (1, reqs["Water"]);
			state = state.WithPopulation (200);
			reqs = engine.GetRequirementsForCurrentPopulation (state);
			Assert.Equal (2, reqs["Water"]);
		}

		[Fact]
		public void PopsActuallyConsumeResourcesOnTick ()
		{
			var engine = CreatePopEngine ();
			var state = CreateGameState ();
			state = state.WithPopulation (100);
			state = state.WithResources (Create ("Water", 2.0));
			state = engine.ProcessTick(state);
			Assert.Equal (1, state.Resources["Water"]);
		}

		[Fact]
		public void PopsDoNotIncreaseLinearly ()
		{
			var engine = CreatePopEngine ();

			// +100
			Assert.Equal (1, engine.GetPopUnitsForTotalPopulation (100));
			Assert.Equal (1.5, engine.GetPopUnitsForTotalPopulation (150));
			Assert.Equal (2, engine.GetPopUnitsForTotalPopulation (200));
			Assert.Equal (4, engine.GetPopUnitsForTotalPopulation (400));
			Assert.Equal (10, engine.GetPopUnitsForTotalPopulation (1000));

			// +200
			Assert.Equal (11, engine.GetPopUnitsForTotalPopulation (1200));
			Assert.Equal (15, engine.GetPopUnitsForTotalPopulation (2000));

			// +500
			Assert.Equal (16, engine.GetPopUnitsForTotalPopulation (2500));
			Assert.Equal (19, engine.GetPopUnitsForTotalPopulation (4000));

			// +1000
			Assert.Equal (20, engine.GetPopUnitsForTotalPopulation (5000));
			Assert.Equal (25, engine.GetPopUnitsForTotalPopulation (10000));

			// +5000
			Assert.Equal (26, engine.GetPopUnitsForTotalPopulation (15000));
			Assert.Equal (32, engine.GetPopUnitsForTotalPopulation (50000));

			// +10000
			Assert.Equal (33, engine.GetPopUnitsForTotalPopulation (60000));
			Assert.Equal (37, engine.GetPopUnitsForTotalPopulation (100000));

			// +50000
			Assert.Equal (38, engine.GetPopUnitsForTotalPopulation (150000));
			Assert.Equal (40, engine.GetPopUnitsForTotalPopulation (250000));
		}

		[Fact]
		public void PopsGrowRateBasedOnSpaceToCap ()
		{
			var engine = CreatePopEngine ();
			double lowRate = engine.GetGrowthRate (100, 200);
			double mideRate = engine.GetGrowthRate (150, 200);
			double highRate = engine.GetGrowthRate (190, 200);
			Assert.True (lowRate > mideRate && mideRate > highRate);

			double lowOverRate = engine.GetGrowthRate (200, 100);
			double mideOverRate = engine.GetGrowthRate (150, 100);
			double highOverRate = engine.GetGrowthRate (110, 100);
			Assert.True (lowOverRate < mideOverRate && mideOverRate < highOverRate);
		}

		[Fact]
		public void GetHousingCapactiy ()
		{
			var engine = CreatePopEngine ();
			var state = CreateGameState (camps: 1);
			Assert.Equal (200, engine.GetHousingCapacity (state));
			state = CreateGameState (camps: 2);
			Assert.Equal (400, engine.GetHousingCapacity (state));
		}

		[Fact]
		public void GetNextAndPreviousBreakpoint ()
		{
			var engine = CreatePopEngine ();
			Assert.Equal (200, engine.GetNextPopBreakpoint (100));
			Assert.Equal (100, engine.GetPreviousPopBreakpoint (100));
			Assert.Equal (900, engine.GetPreviousPopBreakpoint (1000));
			Assert.Equal (1200, engine.GetNextPopBreakpoint (1000));
		}

		[Fact]
		public void PopsGrowIfNeedsMet ()
		{
			var engine = CreatePopEngine ();
			var state = CreateGameState (camps: 1);
			state = state.WithResources (Create ("Water", 200.0));

			double popBefore = state.Population;
			state = engine.ProcessTick (state);
			Assert.True (state.Population > popBefore);
		}

		[Fact]
		public void PopsDecreaseIfNeedsNotMet ()
		{
			var engine = CreatePopEngine ();
			var state = CreateGameState ();
			double popBefore = state.Population;
			state = engine.ProcessTick (state);
			Assert.True (state.Population < popBefore);
		}

		[Fact]
		public void CanNotDecreasePopCapBelowMinimum ()
		{
			var engine = CreatePopEngine ();
			var state = CreateGameState (camps: 2).WithPopulationCap (100);

			Assert.False(engine.CanDecreasePopulationCap (state));
			Assert.Throws<InvalidOperationException> (() => engine.DecreasePopulationCap (state));
		}

		[Fact]
		public void PopsRequireHousingToExpandCap ()
		{
			var engine = CreatePopEngine ();
			var state = CreateGameState (camps: 1);

			Assert.False (engine.CanIncreasePopulationCap (state));
			Assert.Throws<InvalidOperationException> (() => engine.IncreasePopulationCap (state));

			state = CreateGameState (camps: 2);
			Assert.True (engine.CanIncreasePopulationCap (state));
			state = engine.IncreasePopulationCap (state);
			Assert.Equal (300, state.PopulationCap);
		}

		[Fact]
		public void PopsDecreaseIfOverCap ()
		{
			var engine = CreatePopEngine ();
			var state = CreateGameState (camps: 2).WithPopulation (300);
			double popBefore = state.Population;
			state = engine.ProcessTick (state);
			Assert.True (state.Population < popBefore);
		}

		[Fact]
		public void PopsDecreaseIfHousingDestoryed ()
		{
			var engine = CreatePopEngine ();
			var state = CreateGameState (holes: 1).WithPopulation (150);

			var buildingEngine = CreateBuildingEngine ();
			state = buildingEngine.Build (state, state.Regions[0].Name, 0, "Housing");

			double popBefore = state.Population;

			state = buildingEngine.Destroy (state, state.Regions[0].Name, 0, 1);

			state = engine.ProcessTick (state);
			Assert.True (state.Population < popBefore);
		}

		[Fact]
		public void PopsHaveHardMinimumLowerLimit ()
		{
			var engine = CreatePopEngine ();
			var state = CreateGameState ();
			state = state.WithPopulation (100);

			state = engine.ProcessTick (state);
			Assert.Equal (100, state.Population);
			Assert.True (engine.IsPopulationStarving (state));
		}

		[Fact]
		public void PopulationEfficiencyRatios ()
		{
			var engine = CreatePopEngine ();
			double baseValue = engine.GetPopulationEfficiency (4, 5);
			double someOverValue = engine.GetPopulationEfficiency (6, 5);
			double moreOverValue = engine.GetPopulationEfficiency (7, 5);
			Assert.Equal (1.0, baseValue);
			Assert.True (baseValue > someOverValue);
			Assert.True (someOverValue > moreOverValue);
		}

		[Fact]
		public void MoreBuildingsThanPopsReducesEfficiency ()
		{
			var engine = CreatePopEngine ();
			var state = CreateGameState (camps: 1);
			double baseEfficiency = engine.GetPopulationEfficiency (state);
			Assert.Equal (1.0, baseEfficiency);

			state = CreateGameState (camps: 2);
			double lessEfficiency = engine.GetPopulationEfficiency (state);
			Assert.True (lessEfficiency < baseEfficiency);
			Assert.True (lessEfficiency != 0);
		}

		[Fact]
		public void SomeBuildingsDoNotDecreaseEfficiency ()
		{
			ExtraBuildingJSON = @",
			{
				""name"": ""NoJob"",
				""valid_regions"": [""Any""],
				""does_not_require_job"": true
			}";

			var engine = CreatePopEngine ();
			var state = CreateGameState (camps: 1);
			double baseEfficiency = engine.GetPopulationEfficiency (state);
			Assert.Equal (1.0, baseEfficiency);

			var buildingEngine = CreateBuildingEngine ();
			state = buildingEngine.Build (state, state.Regions[0].Name, 0, "NoJob");

			double afterEfficiency = engine.GetPopulationEfficiency (state);
			Assert.Equal (1.0, afterEfficiency);
		}

		[Fact]
		public void ProcessTickGrows ()
		{
			var engine = CreatePopEngine ();
			var state = CreateGameState (camps: 1);
			state = state.WithPopulation (100).WithPopulationCap (200);
			state = engine.ProcessTick (state);
			Assert.True (state.Population > 100 && state.Population < 200);
			for (int i = 0 ; i < 40; ++i)
				state = engine.ProcessTick (state);
			Assert.Equal (200, state.Population);
		}

		[Fact]
		public void PopulationCapGrowsBasedOnBuildings ()
		{
			var engine = CreatePopEngine ();
			var state = CreateGameState (new Area[] { new Area (AreaType.Plains, null), new Area (AreaType.Plains, null), new Area (AreaType.Plains, null) });
			var buildingEngine = CreateBuildingEngine ();
			state = state.WithResources (Create ("Wood", 100));

			// The population cap here is equal, resources and housing
			state = buildingEngine.Build (state, state.Regions[0].Name, 0, "Housing");
			state = buildingEngine.Build (state, state.Regions[0].Name, 0, "Gathering Camp");
			state = state.WithPopulationCap (200);

			// We now have to much housing, so cap is 200
			Assert.Equal (200, engine.FindEffectiveCap (state));
			state = buildingEngine.Build (state, state.Regions[0].Name, 1, "Housing");
			state = state.WithPopulationCap (400);
			Assert.Equal (200, engine.FindEffectiveCap (state));

			// We now have equal again, so 400
			state = buildingEngine.Build (state, state.Regions[0].Name, 1, "Gathering Camp");
			Assert.Equal (400, engine.FindEffectiveCap (state));
		}

		[Fact]
		public void ProcessTickShrinkThenGrows ()
		{
			var engine = CreatePopEngine ();
			var buildingEngine = CreateBuildingEngine ();

			var state = CreateGameState (smokers: 1);
			state = buildingEngine.Build (state, state.Regions[0].Name, 0, "Housing");
			state = state.WithPopulation (100).WithPopulationCap (200);
			state = state.WithResources (Create ("Water", 1.0));

			// Make sure Charcoal conversion doesn't get selected
			state = state.WithResources (Create ("Charcoal", 20.0));

			for (int i = 0 ; i < 40; ++i)
				state = engine.ProcessTick (state);
			Assert.Equal (100, state.Population);

			state = buildingEngine.Build (state, state.Regions[0].Name, 0, "Watering Hole");

			for (int i = 0 ; i < 40; ++i)
				state = engine.ProcessTick (state);
			Assert.Equal (170, state.Population);
		}

		[Fact]
		public void PopulationRequirementsMayChangeDueToTechnology ()
		{
			var engine = CreatePopEngine ();
			var state = CreateGameState ();
			var reqs = engine.GetRequirementsForCurrentPopulation (state);
			Assert.True (reqs["Water"] > 0);
			Assert.True (reqs["Food"] == 0);

			state = state.WithResearchUnlocks (new string [] { "Bronze Age" });

			reqs = engine.GetRequirementsForCurrentPopulation (state);
			Assert.True (reqs["Water"] > 0);
			Assert.True (reqs["Food"] > 0);
		}

		[Fact]
		public void HousingMayChangeDueToTechnology ()
		{
			GameState state = CreateGameState ();
			var buildingEngine = CreateBuildingEngine ();
			state = buildingEngine.Build (state, state.Regions[0].Name, 0, "Housing");

			PopulationEngine engine = CreatePopEngine ();

			Assert.Equal (200, engine.GetHousingCapacity (state));
			state = state.WithResearchUnlocks (new string [] { "Expansion" });
			Assert.Equal (400, engine.GetHousingCapacity (state));
		}
	}
}
