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
		double PopMin;

		public PopulationGrowthCurve (double popMin)
		{
			PopMin = popMin;
		}

		public double GetGrowthRate (double population, bool starved, double effectivePopCap)
		{
			double growthRate = GetBaseGrowthRate (population, effectivePopCap);

			// If we're starving, decrease much faster
			if (starved)
				growthRate *= 5;

			growthRate = RoundGrowthRateAboveMinimumStep (growthRate);
			growthRate = RoundGrowthToPreventOverflow (population, growthRate, effectivePopCap);
			return growthRate;
		}

		public const double MinGrowth = 0.25;
		public double RoundGrowthRateAboveMinimumStep (double growthRate)
		{
			if (growthRate < 0)
				return Math.Min (growthRate, -MinGrowth);
			else
				return Math.Max (growthRate, MinGrowth);
		}

		public double RoundGrowthToPreventOverflow (double population, double growthRate, double effectivePopCap)
		{
			if (growthRate < 0 && population + growthRate < PopMin)
				return PopMin - population;
			else if (growthRate > 0 && population + growthRate > effectivePopCap)
				return effectivePopCap - population;
			return growthRate;
		}

		public double GetBaseGrowthRate (double popSize, double popCap)
		{
			// Logistic growth
			const double R = .025;
			return R * ((popCap - popSize) / popSize) * popSize;
		}
	}
}