using System;
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

		}

		[Fact]
		public void PopsGrowRateBasedOnSpaceToCap ()
		{
			double lowRate = PopulationEngine.GetGrowthRate (100, 200);
			double mideRate = PopulationEngine.GetGrowthRate (150, 200);
			double highRate = PopulationEngine.GetGrowthRate (190, 200);
			Assert.True (lowRate > mideRate && mideRate > highRate);
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