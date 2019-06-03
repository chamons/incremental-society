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
		PopulationCapacity PopulationCapacity;

		public PopulationGrowthCurve (PopulationCapacity populationCapacity, double popMin)
		{
			PopulationCapacity = populationCapacity;
			PopMin = popMin;
		}

		public double GetGrowthRate (GameState state, PopulationRatio happy, PopulationRatio health)
		{
			double effectivePopCap = PopulationCapacity.FindEffectiveCap (state);

			double growthRate = CalculatePopulationGrowthRate (state.Population, happy);
			double freeHousing = PopulationCapacity.GetHousingCapacity (state) - state.Population;
			growthRate += CalculateImmigrationRate (freeHousing, happy);
			growthRate -= CalculateEmmigrationRate (state.Population, happy);
			growthRate -= CalculatePopulationDeathRate (state.Population, health);

			growthRate = RoundGrowthRateAboveMinimumStep (growthRate);
			growthRate = RoundGrowthToPreventOverflow (state.Population, growthRate, effectivePopCap);
			return growthRate;
		}

		public const double MinGrowth = 0.2;
		public double RoundGrowthRateAboveMinimumStep (double growthRate)
		{
			if (growthRate < 0)
				return Math.Min (growthRate, -MinGrowth);
			else
				return Math.Max (growthRate, MinGrowth);
		}

		public double RoundGrowthToPreventOverflow (double population, double growthRate, double effectivePopCap)
		{
			double expectedPopulation = population + growthRate;

			// If it pushes us above pop cap, stop there
			if (expectedPopulation > effectivePopCap) {
				growthRate = effectivePopCap - population;
				expectedPopulation = population + growthRate;
			}

			// If our expected rate pulls us under PopMin
			if (expectedPopulation < PopMin)
				growthRate = PopMin - population;

			return growthRate;
		}

		const double BasePopGrowthRate = .01;
		public double CalculatePopulationGrowthRate (double population, PopulationRatio happiness)
		{
			return population * BasePopGrowthRate * (happiness.Value * .8 + .2);
		}

		const double BaseImmigrationRate = .01;
		public double CalculateImmigrationRate (double freeHousing, PopulationRatio happiness)
		{
			if (happiness.Value <= .5)
				return 0;
			return freeHousing * BaseImmigrationRate * ((happiness.Value - .5) * 2);
		}

		const double BaseEmmigrationRate = .01;
		public double CalculateEmmigrationRate (double population, PopulationRatio happiness)
		{
			if (happiness.Value >= .5)
				return 0;
			return population * BaseEmmigrationRate * (.5 - happiness.Value) * 2;
		}

		const double BaseDeathRate = .005;
		public double CalculatePopulationDeathRate (double population, PopulationRatio health)
		{
			return population * BaseDeathRate * (3 + (health.Value * -2));
		}
	}
}