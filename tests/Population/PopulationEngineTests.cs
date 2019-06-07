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
	public class PopulationEngineTests : ResourceTestBase
	{
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
		}

		[Fact]
		public void ProcessTickGrows ()
		{
			var engine = CreatePopEngine ();
			var state = CreateGameState (camps: 1);
			state = state.WithPopulation (100).WithPopulationCap (200);
			state = engine.ProcessTick (state);
			Assert.True (state.Population > 100 && state.Population < 200);
			for (int i = 0 ; i < 200; ++i)
				state = engine.ProcessTick (state);
			Assert.Equal (200, state.Population);
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

			for (int i = 0 ; i < 100; ++i)
				state = engine.ProcessTick (state);
			Assert.Equal (100, state.Population);

			state = buildingEngine.Destroy (state, state.Regions[0].Name, 0, 0);
			state = buildingEngine.Build (state, state.Regions[0].Name, 0, "Watering Hole");

			for (int i = 0 ; i < 100; ++i)
				state = engine.ProcessTick (state);
			Assert.Equal (170, state.Population);
		}
	}
}
