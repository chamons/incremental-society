using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Text;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

namespace IncrementalSociety.Population
{
	public class PopulationGrowthCurve
	{
		double PopMin;
		PopulationCapacity PopulationCapacity;

		public double MinGrowth;
		double BasePopGrowthRate;
		double BaseImmigrationRate;
		double BaseEmmigrationRate;
		double HousingEmmigrationRate;
		double BaseDeathRate;

		public PopulationGrowthCurve (PopulationCapacity populationCapacity, JsonLoader json)
		{
			PopulationCapacity = populationCapacity;
			PopMin = json.Game.MinPopulation;
			MinGrowth = json.Game.MinGrowth;
			BasePopGrowthRate = json.Game.BasePopGrowthRate;
			BaseImmigrationRate = json.Game.BaseImmigrationRate;
			BaseEmmigrationRate = json.Game.BaseEmmigrationRate;
			HousingEmmigrationRate = json.Game.HousingEmmigrationRate;
			BaseDeathRate = json.Game.BaseDeathRate;
		}

		public double GetGrowthRate (GameState state, PopulationRatio happy, PopulationRatio health)
		{
			double effectivePopCap = PopulationCapacity.FindEffectiveCap (state);
			double freeHousing = PopulationCapacity.GetHousingCapacity (state) - state.Population;

			double popGrowthRate = CalculatePopulationGrowthRate (state.Population, happy);
			double immigrationRate = CalculateImmigrationRate (freeHousing, happy);
			double emmigrationRate = CalculateEmmigrationRate (state.Population, happy, freeHousing);
			double deathRate = CalculatePopulationDeathRate (state.Population, health);

			double growthRate = popGrowthRate + immigrationRate - emmigrationRate - deathRate;

			growthRate = RoundGrowthRateAboveMinimumStep (growthRate);
			growthRate = RoundGrowthToPreventOverflow (state.Population, growthRate, effectivePopCap);
			return growthRate;
		}

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

			// If we have a positive growth rate and we'll overshoot reduce towards zero
			if (expectedPopulation > effectivePopCap && growthRate > 0)
				return RoundGrowthToPreventOverflow (population, Math.Max (0, effectivePopCap - population), effectivePopCap);

			// If we have a negative growth and we'll undershoot min, reduce to hit it directly
			if (expectedPopulation < PopMin && growthRate < 0)
				return PopMin - population;

			return growthRate;
		}

		public double CalculatePopulationGrowthRate (double population, PopulationRatio happiness)
		{
			return population * BasePopGrowthRate * (happiness.Value * .8 + .2);
		}

		public double CalculateImmigrationRate (double freeHousing, PopulationRatio happiness)
		{
			// No one wants to immigrate to an unhappy land or one without space
			if (happiness.Value <= .5 || freeHousing < 1)
				return 0;
			return freeHousing * BaseImmigrationRate * ((happiness.Value - .5) * 2);
		}

		public double CalculateEmmigrationRate (double population, PopulationRatio happiness, double freeHousing)
		{
			double happinessEmmigration = 0;
			if (happiness.Value < .5)
				happinessEmmigration = population * BaseEmmigrationRate * (.5 - happiness.Value) * 2;

			double spaceEmmigration = 0;
			if (freeHousing < 0)
				spaceEmmigration = -1 * freeHousing * HousingEmmigrationRate;
			return happinessEmmigration + spaceEmmigration;
		}

		public double CalculatePopulationDeathRate (double population, PopulationRatio health)
		{
			return population * BaseDeathRate * (3 + (health.Value * -2));
		}
	}
}