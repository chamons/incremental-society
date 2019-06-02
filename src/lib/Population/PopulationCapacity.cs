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
	public class PopulationCapacity
	{
		double PopMin;
		ResourceEngine ResourceEngine;
		PopulationResources PopulationResources;

		public PopulationCapacity (ResourceEngine resourceEngine, PopulationResources populationResourceFinder, double popMin)
		{
			ResourceEngine = resourceEngine;
			PopulationResources = populationResourceFinder;
			PopMin = popMin;
		}

		public double FindEffectiveCap (GameState state)
		{
			var resourcesPerTick = ResourceEngine.CalculateAdditionalNextTick (state, 1.0);

			double effectivePopCap;
			if (resourcesPerTick.HasMoreThan (PopulationResources.GetRequirementsForPopulation (state, state.PopulationCap)))
				effectivePopCap = state.PopulationCap;
			else
				effectivePopCap = PopulationResources.FindResourceEffectivePopCap (state, resourcesPerTick);

			// If our housing is lower than income, use that as effective cap
			effectivePopCap = Math.Min (effectivePopCap, GetHousingCapacity (state));

			// We've divided by zero multiple times in the past, let's sanity check in debug
#if DEBUG
			if (!effectivePopCap.HasValue())
				throw new InvalidOperationException ($"Processing population tick produced invalid population cap: {effectivePopCap}");
#endif

			return effectivePopCap;
		}

		public double GetHousingCapacity (GameState state) => state.AllBuildings ().Sum (x => ResourceEngine.GetBuildingHousing (state, x));

		public bool CanIncreasePopulationCap (GameState state)
		{
			return GetHousingCapacity (state) >= GetNextPopBreakpoint (state.PopulationCap);
		}

		public GameState IncreasePopulationCap (GameState state)
		{
			if (!CanIncreasePopulationCap (state))
				throw new InvalidOperationException ($"Unable to increase pop cap {state.PopulationCap}");
			return state.WithPopulationCap (GetNextPopBreakpoint (state.PopulationCap));
		}

		public bool CanDecreasePopulationCap (GameState state)
		{
			return state.PopulationCap != PopMin;
		}

		public GameState DecreasePopulationCap (GameState state)
		{
			if (!CanDecreasePopulationCap (state))
				throw new InvalidOperationException ($"Unable to decrease pop cap {state.PopulationCap}");
			return state.WithPopulationCap (GetPreviousPopBreakpoint (state.PopulationCap));
		}

		public double GetPopUnitsForTotalPopulation (double population)
		{
			if (population < 1000) {
				return population / 100;
			} else if (population < 2000) {
				return 10 + (population - 1000) / 200;
			} else if (population < 4000) {
				return 15 + (population - 2000) / 500;
			} else if (population < 10000) {
				return 19 + (population - 4000) / 1000;
			} else if (population < 50000) {
				return 25 + (population - 10000) / 5000;
			} else if (population< 100000) {
				return 32 + (population - 50000) / 10000;
			} else {
				return 37 + (population - 100000) / 50000;
			}
		}

		public double GetNextPopBreakpoint (double current)
		{
			if (current < 1000)
				return current + 100;
			else if (current < 2000)
				return current + 200;
			else if (current < 4000)
				return current + 500;
			else if (current < 10000)
				return current + 1000;
			else if (current < 50000)
				return current + 5000;
			else if (current < 100000)
				return current + 10000;
			else
				return current + 50000;
		}

		public double GetPreviousPopBreakpoint (double current)
		{
			if (current == PopMin)
				return current;

			if (current <= 1000)
				return current - 100;
			else if (current <= 2000)
				return current - 200;
			else if (current <= 4000)
				return current - 500;
			else if (current <= 10000)
				return current - 1000;
			else if (current <= 50000)
				return current - 5000;
			else if (current <= 100000)
				return current - 10000;
			else
				return current - 50000;
		}

	}
}