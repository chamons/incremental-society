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
	public class PopulationCapacityTests : ResourceTestBase
	{
		[Fact]
		public void PopsDoNotIncreaseLinearly ()
		{
			var capacity = CreatePopulationCapacity ();

			// +100
			Assert.Equal (1, capacity.GetPopUnitsForTotalPopulation (100));
			Assert.Equal (1.5, capacity.GetPopUnitsForTotalPopulation (150));
			Assert.Equal (2, capacity.GetPopUnitsForTotalPopulation (200));
			Assert.Equal (4, capacity.GetPopUnitsForTotalPopulation (400));
			Assert.Equal (10, capacity.GetPopUnitsForTotalPopulation (1000));

			// +200
			Assert.Equal (11, capacity.GetPopUnitsForTotalPopulation (1200));
			Assert.Equal (15, capacity.GetPopUnitsForTotalPopulation (2000));

			// +500
			Assert.Equal (16, capacity.GetPopUnitsForTotalPopulation (2500));
			Assert.Equal (19, capacity.GetPopUnitsForTotalPopulation (4000));

			// +1000
			Assert.Equal (20, capacity.GetPopUnitsForTotalPopulation (5000));
			Assert.Equal (25, capacity.GetPopUnitsForTotalPopulation (10000));

			// +5000
			Assert.Equal (26, capacity.GetPopUnitsForTotalPopulation (15000));
			Assert.Equal (32, capacity.GetPopUnitsForTotalPopulation (50000));

			// +10000
			Assert.Equal (33, capacity.GetPopUnitsForTotalPopulation (60000));
			Assert.Equal (37, capacity.GetPopUnitsForTotalPopulation (100000));

			// +50000
			Assert.Equal (38, capacity.GetPopUnitsForTotalPopulation (150000));
			Assert.Equal (40, capacity.GetPopUnitsForTotalPopulation (250000));
		}

		[Fact]
		public void GetHousingCapactiy ()
		{
			var capacity = CreatePopulationCapacity ();

			var state = CreateGameState (camps: 1);
			Assert.Equal (200, capacity.GetHousingCapacity (state));
			state = CreateGameState (camps: 2);
			Assert.Equal (400, capacity.GetHousingCapacity (state));
		}

		[Fact]
		public void GetNextAndPreviousBreakpoint ()
		{
			var capacity = CreatePopulationCapacity ();

			Assert.Equal (200, capacity.GetNextPopBreakpoint (100));
			Assert.Equal (100, capacity.GetPreviousPopBreakpoint (100));
			Assert.Equal (900, capacity.GetPreviousPopBreakpoint (1000));
			Assert.Equal (1200, capacity.GetNextPopBreakpoint (1000));
		}

		[Fact]
		public void CanNotDecreasePopCapBelowMinimum ()
		{
			var capacity = CreatePopulationCapacity ();
			var state = CreateGameState (camps: 2).WithPopulationCap (100);

			Assert.False(capacity.CanDecreasePopulationCap (state));
			Assert.Throws<InvalidOperationException> (() => capacity.DecreasePopulationCap (state));
		}

		[Fact]
		public void PopsRequireHousingToExpandCap ()
		{
			var capacity = CreatePopulationCapacity ();
			var state = CreateGameState (camps: 1);

			Assert.False (capacity.CanIncreasePopulationCap (state));
			Assert.Throws<InvalidOperationException> (() => capacity.IncreasePopulationCap (state));

			state = CreateGameState (camps: 2);
			Assert.True (capacity.CanIncreasePopulationCap (state));
			state = capacity.IncreasePopulationCap (state);
			Assert.Equal (300, state.PopulationCap);
		}

		[Fact]
		public void HousingMayChangeDueToTechnology ()
		{
			GameState state = CreateGameState ();
			var buildingEngine = CreateBuildingEngine ();
			state = buildingEngine.Build (state, state.Regions[0].Name, 0, "Housing");

			var capacity = CreatePopulationCapacity ();

			Assert.Equal (200, capacity.GetHousingCapacity (state));
			state = state.WithResearchUnlocks (new string [] { "Expansion" });
			Assert.Equal (400, capacity.GetHousingCapacity (state));
		}

		[Fact]
		public void PopulationCapGrowsBasedOnBuildings ()
		{
			var capacity = CreatePopulationCapacity ();
			var state = CreateGameState (new Area[] { new Area ("Plains", null), new Area ("Plains", null), new Area ("Plains", null) });
			var buildingEngine = CreateBuildingEngine ();
			state = state.WithResources (Create ("Wood", 100));

			// The population cap here is equal, resources and housing
			state = buildingEngine.Build (state, state.Regions[0].Name, 0, "Housing");
			state = buildingEngine.Build (state, state.Regions[0].Name, 0, "Gathering Camp");
			state = state.WithPopulationCap (200);

			// We now have to much housing, so cap is 200
			Assert.Equal (200, capacity.FindEffectiveCap (state));
			state = buildingEngine.Build (state, state.Regions[0].Name, 1, "Housing");
			state = state.WithPopulationCap (400);
			Assert.Equal (200, capacity.FindEffectiveCap (state));

			// We now have equal again, so 400
			state = buildingEngine.Build (state, state.Regions[0].Name, 1, "Gathering Camp");
			Assert.Equal (400, capacity.FindEffectiveCap (state));
		}
	}
}
