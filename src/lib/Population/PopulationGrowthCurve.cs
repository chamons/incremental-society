using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Text;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

namespace IncrementalSociety.Population
{
	public class PopulationGrowthCurve
	{
		public PopulationGrowthCurve ()
		{
		}

		public double GetGrowthRate (double popSize, double popCap)
		{
			return GetBaseGrowthRate (popSize, popCap);
		}

		public double GetBaseGrowthRate (double popSize, double popCap)
		{
			// Logistic growth
			const double R = .025;
			return R * ((popCap - popSize) / popSize) * popSize;
		}
	}
}