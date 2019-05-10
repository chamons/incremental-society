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
			var reqs = engine.GetFullRequirementsForNextTick (state);
			Assert.Equal (1000, reqs.AmountOf ("water"));
			state = state.WithPopulation (2000);
			reqs = engine.GetFullRequirementsForNextTick (state);
			Assert.Equal (2000, reqs.AmountOf ("water"));
		}

		[Fact]
		public void PopsProvideBuildingCapacity ()
		{

		}

		[Fact]
		public void PopsRequireHousingToExpandCap ()
		{

		}

		[Fact]
		public void PopsDoNotIncreaseLinearly ()
		{
			var engine = Factories.CreatePopEngine ();

			Assert.Equal (1, engine.GetPopUnitsForTotalPopulation (1000));
			Assert.Equal (2, engine.GetPopUnitsForTotalPopulation (2000));
			Assert.Equal (4, engine.GetPopUnitsForTotalPopulation (4000));
			Assert.Equal (10, engine.GetPopUnitsForTotalPopulation (10000));
			Assert.Equal (11, engine.GetPopUnitsForTotalPopulation (12000));
			Assert.Equal (15, engine.GetPopUnitsForTotalPopulation (20000));
			Assert.Equal (16, engine.GetPopUnitsForTotalPopulation (25000));
			Assert.Equal (19, engine.GetPopUnitsForTotalPopulation (40000));
			Assert.Equal (20, engine.GetPopUnitsForTotalPopulation (50000));
			Assert.Equal (25, engine.GetPopUnitsForTotalPopulation (100000));
			Assert.Equal (26, engine.GetPopUnitsForTotalPopulation (200000));
			Assert.Equal (28, engine.GetPopUnitsForTotalPopulation (400000));
		}

		[Fact]
		public void PopsGrowRateBasedOnSpaceToCap ()
		{
			double lowRate = PopulationEngine.GetGrowthRate (100, 200);
			double mideRate = PopulationEngine.GetGrowthRate (150, 200);
			double highRate = PopulationEngine.GetGrowthRate (190, 200);
			Assert.True (lowRate > mideRate && mideRate > highRate);

			double lowOverRate = PopulationEngine.GetGrowthRate (200, 100);
			double mideOverRate = PopulationEngine.GetGrowthRate (150, 100);
			double highOverRate = PopulationEngine.GetGrowthRate (110, 100);
			Assert.True (lowOverRate < mideOverRate && mideOverRate < highOverRate);
		}

		[Fact]
		public void PopsGrowToCapIfNeedsMet ()
		{

		}

		[Fact]
		public void PopsDecreaseIfNeedsNotMet ()
		{

		}

		[Fact]
		public void PopsDecreaseIfOverHousingCap ()
		{

		}

		[Fact]
		public void PopsDecreaseGlobalEffeciencyIfTooFew ()
		{

		}

		[Fact]
		public void PopsHaveHardMinimumLowerLimit ()
		{

		}
	}
}