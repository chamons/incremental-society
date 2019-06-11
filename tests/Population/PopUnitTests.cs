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
	public class PopUnitTests : ResourceTestBase
	{
		[Fact]
		public void PopsDoNotIncreaseLinearly ()
		{
			var units = CreatePopUnits ();

			// +100
			Assert.Equal (1, units.GetPopUnitsForTotalPopulation (100));
			Assert.Equal (1.5, units.GetPopUnitsForTotalPopulation (150));
			Assert.Equal (2, units.GetPopUnitsForTotalPopulation (200));
			Assert.Equal (4, units.GetPopUnitsForTotalPopulation (400));
			Assert.Equal (10, units.GetPopUnitsForTotalPopulation (1000));

			// +200
			Assert.Equal (11, units.GetPopUnitsForTotalPopulation (1200));
			Assert.Equal (15, units.GetPopUnitsForTotalPopulation (2000));

			// +500
			Assert.Equal (16, units.GetPopUnitsForTotalPopulation (2500));
			Assert.Equal (19, units.GetPopUnitsForTotalPopulation (4000));

			// +1000
			Assert.Equal (20, units.GetPopUnitsForTotalPopulation (5000));
			Assert.Equal (25, units.GetPopUnitsForTotalPopulation (10000));

			// +5000
			Assert.Equal (26, units.GetPopUnitsForTotalPopulation (15000));
			Assert.Equal (32, units.GetPopUnitsForTotalPopulation (50000));

			// +10000
			Assert.Equal (33, units.GetPopUnitsForTotalPopulation (60000));
			Assert.Equal (37, units.GetPopUnitsForTotalPopulation (100000));

			// +50000
			Assert.Equal (38, units.GetPopUnitsForTotalPopulation (150000));
			Assert.Equal (40, units.GetPopUnitsForTotalPopulation (250000));
		}

		[Fact]
		public void GetNextAndPreviousBreakpoint ()
		{
			var units = CreatePopUnits ();

			Assert.Equal (200, units.GetNextPopBreakpoint (100));
			Assert.Equal (100, units.GetPreviousPopBreakpoint (100));
			Assert.Equal (900, units.GetPreviousPopBreakpoint (1000));
			Assert.Equal (1200, units.GetNextPopBreakpoint (1000));
		}
	}
}
