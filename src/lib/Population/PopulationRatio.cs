using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Text;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

namespace IncrementalSociety.Population
{
	public class PopulationRatio
	{
		public double Value { get; }

		public PopulationRatio (double value)
		{
#if DEBUG
			if (value < 0 || value > 1)
				throw new InvalidOperationException ($"PopulationRatio with invalid value of {value}");
#endif
			Value = value;
		}

		public static PopulationRatio Create (double value) => new PopulationRatio (value);
	}
}