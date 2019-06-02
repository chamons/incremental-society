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

		public double GetGrowthRate (GameState state, bool starved, double effectivePopCap)
		{
			double baseRate = GetBaseGrowthRate (state.Population, effectivePopCap);
			return TweakGrowthRate (state, starved, baseRate, effectivePopCap);
		}

		double TweakGrowthRate (GameState state, bool starved, double growthRate, double effectivePopCap)
		{
			// If we're starving, decrease much faster
			// If we're within one of the cap, round our rate to make a nice .25
			// Else if our rate is less than one, round "up/down" to prevent very small changes from taking forever
			if (starved)
				growthRate *= 5;

			const double MinGrowth = 0.25;
			if (growthRate < 0) {
				if (state.Population - effectivePopCap < MinGrowth)
					return effectivePopCap - state.Population;
				else
					return Math.Min (growthRate, -MinGrowth);
			}
			else {
				if (effectivePopCap - state.Population < MinGrowth)
					return effectivePopCap - state.Population;
				else
					return Math.Max (growthRate, MinGrowth);
			}
		}

		public double GetBaseGrowthRate (double popSize, double popCap)
		{
			// Logistic growth
			const double R = .025;
			return R * ((popCap - popSize) / popSize) * popSize;
		}
	}
}