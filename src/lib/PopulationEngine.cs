using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Text;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

namespace IncrementalSociety
{
	public class PopulationEngine
	{
		ResourceEngine ResourceEngine;

		JsonLoader Json;
		double PopMin;

		ResourceConfig ResourceConfig => ResourceEngine.ResourceConfig;

		public PopulationEngine (ResourceEngine resourceEngine, JsonLoader json)
		{
			ResourceEngine = resourceEngine;
			Json = json;
			PopMin = json.Game.MinPopulation;
		}

		public Resources GetRequirementsForPopulation (GameState state)
		{
			return GetRequirementsPerPop (state).Multiply (state.Population);
		}

		public Resources GetRequirementsPerPop (GameState state)
		{
			return ResourceEngine.GetResourcesBasedOnTech (state, Json.Game.PopulationNeeds);
		}

		public GameState ProcessTick (GameState state)
		{
			// Step 0 - Determine how many people our income and housing supports
			double effectivePopCap = FindEffectiveCap (state);
#if DEBUG
			if (!effectivePopCap.HasValue())
				throw new InvalidOperationException ($"Processing population tick produced invalid population cap: {effectivePopCap}");
#endif

			// Step 1 - Determine how many resources we need for our current population
			var neededResource = GetRequirementsForPopulation (state);

			// Step 2 - Consume if we have enough, else consume as much as we can
			bool starved = !state.Resources.HasMoreThan (neededResource);
			state = ConsumeResources (state, neededResource);

			// Step 3a Get new desired growth rate
			double growthRate = GetGrowthRate (state.Population, effectivePopCap);

			// Step 3b If we starved some people, multiple negative by x5
			if (starved)
				growthRate *= 5;

			// Step 3c Tweak the growth rate to be "nicer":
			// - If we're within one of the cap, round our rate to make a nice .25
			// - Else if our rate is less than one, round "up/down" to prevent very small changes from taking forever
			const double MinGrowth = 0.25;
			if (growthRate < 0) {
				if (state.Population - effectivePopCap < MinGrowth)
					growthRate = effectivePopCap - state.Population;
				else
					growthRate = Math.Min (growthRate, -MinGrowth);
			}
			else {
				if (effectivePopCap - state.Population < MinGrowth)
					growthRate = effectivePopCap - state.Population;
				else
					growthRate = Math.Max (growthRate, MinGrowth);
			}

			// Step 3d If growing, don't grow over our effectice cap because then we'll just starve later
			if (growthRate > 0 && state.Population + growthRate > effectivePopCap)
				return state;

			// Step 4 Grow!
			return GrowAtRate (state, growthRate, effectivePopCap);
		}

		GameState ConsumeResources (GameState state, Resources consumedResources)
		{
			var currentResources = state.Resources.ToBuilder ();
			currentResources.Subtract (consumedResources);

			// Don't go negative when consuming population resources
			for (int i = 0 ; i < ResourceConfig.ResourceLength ; ++i) {
				if (currentResources [i] < 0)
					currentResources [i] = 0;
			}

			return state.WithResources (currentResources);
		}

		public double FindEffectiveCap (GameState state)
		{
			var resourcesPerTick = ResourceEngine.CalculateAdditionalNextTick (state, 1.0);

			double effectivePopCap;
			if (resourcesPerTick.HasMoreThan (GetRequirementsForPopulation (state)))
				effectivePopCap = state.PopulationCap;
			else
				effectivePopCap = FindResourceEffectivePopCap (state, resourcesPerTick);

			// If our housing is lower than income, use that as effective cap
			effectivePopCap = Math.Min (effectivePopCap, GetHousingCapacity (state));
			return effectivePopCap;
		}

		public bool IsPopulationStarving (GameState state)
		{
			var neededResource = GetRequirementsForPopulation (state);
			var nextTickResources = state.Resources.ToBuilder ();
			nextTickResources.Add (ResourceEngine.CalculateAdditionalNextTick (state, 1.0));
			return !nextTickResources.HasMoreThan (neededResource);
		}

		string FindMostMissingResource (GameState state, Resources resourcesPerTick)
		{
			var delta = resourcesPerTick.Subtract (GetRequirementsForPopulation (state)).ToBuilder ();
			var popNeed = GetRequirementsForPopulation (state);

			foreach (var need in popNeed)
				delta[need.ResourceName] = delta[need.ResourceName] / need.Value;
			return delta.OrderBy (x => x.Value).Select (x => x.ResourceName).Where (x => popNeed[x] > 0).First ();
		}

		double FindResourceEffectivePopCap (GameState state, Resources resourcesPerTick)
		{
			string mostMissingResource = FindMostMissingResource (state, resourcesPerTick);
			var totalPopNeed = GetRequirementsForPopulation (state);
			var needPerPop = GetRequirementsPerPop (state);
			var delta = resourcesPerTick.ToBuilder ();
			delta.Subtract (totalPopNeed);
			double peopleShort = delta[mostMissingResource] / needPerPop[mostMissingResource];
			return state.Population + peopleShort;
		}

		GameState GrowAtRate (GameState state, double rate, double cap)
		{
			double newPopulation;
			if (rate > 0)
				newPopulation = Math.Min (state.Population + rate, cap);
			else
				newPopulation = Math.Max (state.Population + rate, PopMin);

			return state.WithPopulation (newPopulation);
		}

		public double GetHousingCapacity (GameState state) => state.AllBuildings ().Sum (x => ResourceEngine.FindBuilding (x).HousingCapacity);

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

		public double GetGrowthRate (double popSize, double popCap)
		{
			// Logistic growth
			const double R = .025;
			return R * ((popCap - popSize) / popSize) * popSize;
		}

		public int GetBuildingJobCount (GameState state)
		{
			return state.AllBuildings ().Where (x => !ResourceEngine.FindBuilding (x).DoesNotRequireJob).Count ();
		}

		public double GetPopulationEfficiency (GameState state)
		{
			int buildingCount = GetBuildingJobCount (state);
			double totalPopCount = GetPopUnitsForTotalPopulation (state.Population);
			return GetPopulationEfficiency (buildingCount, totalPopCount);
		}

		public double GetPopulationEfficiency (int buildingCount, double totalPopCount)
		{
			if (totalPopCount >= buildingCount)
				return 1.0;
			else
				return 1.0 - ((double)(buildingCount - totalPopCount) / (double)buildingCount);
		}
	}
}
