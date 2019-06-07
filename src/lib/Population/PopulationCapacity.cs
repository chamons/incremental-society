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
		ResourceEngine ResourceEngine;
		PopulationResources PopulationResources;
		PopulationBuildingInfo PopulationBuildingInfo;

		PopUnits PopUnits;
		double PopMin => PopUnits.PopMin;

		public PopulationCapacity (ResourceEngine resourceEngine, PopulationResources populationResourceFinder, PopulationBuildingInfo populationBuildingInfo, PopUnits popUnits)
		{
			ResourceEngine = resourceEngine;
			PopulationResources = populationResourceFinder;
			PopulationBuildingInfo = populationBuildingInfo;
			PopUnits = popUnits;
		}

		public double FindEffectiveCap (GameState state)
		{
			double efficiency = PopulationBuildingInfo.GetPopulationEfficiency (state);
			var resourcesPerTick = ResourceEngine.CalculateAdditionalNextTick (state, efficiency);

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
			return GetHousingCapacity (state) >= PopUnits.GetNextPopBreakpoint (state.PopulationCap);
		}

		public GameState IncreasePopulationCap (GameState state)
		{
			if (!CanIncreasePopulationCap (state))
				throw new InvalidOperationException ($"Unable to increase pop cap {state.PopulationCap}");
			return state.WithPopulationCap (PopUnits.GetNextPopBreakpoint (state.PopulationCap));
		}

		public bool CanDecreasePopulationCap (GameState state)
		{
			return state.PopulationCap != PopMin;
		}

		public GameState DecreasePopulationCap (GameState state)
		{
			if (!CanDecreasePopulationCap (state))
				throw new InvalidOperationException ($"Unable to decrease pop cap {state.PopulationCap}");
			return state.WithPopulationCap (PopUnits.GetPreviousPopBreakpoint (state.PopulationCap));
		}
	}
}