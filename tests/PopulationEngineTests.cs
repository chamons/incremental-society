using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

using Xunit;

namespace IncrementalSociety.Tests
{
	public class PopulationEngineTests
	{
		[Fact]
		public void PopsRequireResourcesEachTick ()
		{
			var engine = Factories.CreatePopEngine ();
			var state = Factories.CreateGameState ();
			state = state.WithPopulation (100);

			var reqs = engine.GetFullRequirementsForNextTick (state);
			Assert.Equal (100, reqs.AmountOf ("Water"));
			state = state.WithPopulation (200);
			reqs = engine.GetFullRequirementsForNextTick (state);
			Assert.Equal (200, reqs.AmountOf ("Water"));
		}
		
		[Fact]
		public void PopsActuallyConsumeResourcesOnTick ()
		{
			var engine = Factories.CreatePopEngine ();
			var state = Factories.CreateGameState ();
			state = state.WithPopulation (100);
			state = state.WithResources (Immutable.CreateDictionary ("Water", 200.0));
			state = engine.ProcessTick(state);
			Assert.Equal (100, state.Resources.AmountOf ("Water"));
		}

		[Fact]
		public void PopsDoNotIncreaseLinearly ()
		{
			var engine = Factories.CreatePopEngine ();

			// +100
			Assert.Equal (1, engine.GetPopUnitsForTotalPopulation (100));
			Assert.Equal (1, engine.GetPopUnitsForTotalPopulation (150));
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
			var engine = Factories.CreatePopEngine ();
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
		public void PopsGrowToCapIfNeedsMet ()
		{
			var engine = Factories.CreatePopEngine ();
			var state = Factories.CreateGameState ();
			state = state.WithResources (Immutable.CreateDictionary ("Water", 200.0));

			double popBefore = state.Population;
			state = engine.ProcessTick (state);
			Assert.True (state.Population > popBefore);
		}

		[Fact]
		public void PopsDecreaseIfNeedsNotMet ()
		{
			var engine = Factories.CreatePopEngine ();
			var state = Factories.CreateGameState ();
			double popBefore = state.Population;
			state = engine.ProcessTick (state);
			Assert.True (state.Population < popBefore);
		}

		[Fact]
		public void CanNotDecreasePopCapBelowMinimum ()
		{
			var engine = Factories.CreatePopEngine ();
			var state = Factories.CreateGameState (camps: 2).WithPopulationCap (100);

			Assert.False(engine.CanDecreasePopulationCap (state));
			Assert.Throws<InvalidOperationException> (() => engine.DecreasePopulationCap (state));
		}

		[Fact]
		public void PopsRequireHousingToExpandCap ()
		{
			var engine = Factories.CreatePopEngine ();
			var state = Factories.CreateGameState (camps: 1);

			Assert.False (engine.CanIncreasePopulationCap (state));
			Assert.Throws<InvalidOperationException> (() => engine.IncreasePopulationCap (state));

			state = Factories.CreateGameState (camps: 2);
			Assert.True (engine.CanIncreasePopulationCap (state));
			state = engine.IncreasePopulationCap (state);
			Assert.Equal (300, state.PopulationCap);
		}

		[Fact]
		public void PopsDecreaseIfOverCap ()
		{
			var engine = Factories.CreatePopEngine ();
			var state = Factories.CreateGameState (camps: 2).WithPopulation (300);
			double popBefore = state.Population;
			state = engine.ProcessTick (state);
			Assert.True (state.Population < popBefore);
		}

		[Fact]
		public void PopsHaveHardMinimumLowerLimit ()
		{
			var engine = Factories.CreatePopEngine ();
			var state = Factories.CreateGameState ();
			state = state.WithPopulation (100);

			state = engine.ProcessTick (state);
			Assert.Equal (100, state.Population);
		}

		[Fact]
		public void PopulationEfficiencyRatios ()
		{
			var engine = Factories.CreatePopEngine ();
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
			var engine = Factories.CreatePopEngine ();
			var state = Factories.CreateGameState (camps: 1);
			double baseEfficiency = engine.GetPopulationEfficiency (state);
			Assert.Equal (1.0, baseEfficiency);

			state = Factories.CreateGameState (camps: 2);
			double lessEfficiency = engine.GetPopulationEfficiency (state);
			Assert.True (lessEfficiency < baseEfficiency);
			Assert.True (lessEfficiency != 0);
		}
	}
}
