using System;
using System.Collections.Generic;
using System.Text;

namespace IncrementalSociety
{
	public class PopulationEngine
	{
		public static double GetGrowthRate (double popSize, double popCap)
		{
			// Logistic growth
			const double R = .05;
			return R * ((popCap - popSize) / popSize) * popSize;
		}
	}
}
