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
		ImmutableDictionary<string, double> PopNeed;
		HashSet <string> PopNeedNames;
		double PopMin;
		YieldCache Yields;

		public PopulationEngine (ResourceEngine resourceEngine, JsonLoader json)
		{
			ResourceEngine = resourceEngine;
			Yields = new YieldCache ();
			LoadAndCalculatePopNeed (json);
		}

		void LoadAndCalculatePopNeed (JsonLoader json)
		{
			var totalNeed = ImmutableDictionary.CreateBuilder<string, double> ();
			totalNeed.Add (Yields.Total (json.Game.PopulationNeeds));
			PopNeed = totalNeed.ToImmutable ();
			PopNeedNames = new HashSet<string> (PopNeed.Keys);

			PopMin = json.Game.MinPopulation;
		}
		
		public ImmutableDictionary<string, double> GetRequirementsForPopulation (GameState state) => GetRequirementsForPopulation (state.Population);

		public ImmutableDictionary<string, double> GetRequirementsForPopulation (double population)
		{
			var amount = PopNeed.ToBuilder ();
			amount.Multiply (population);
			return amount.ToImmutable ();
		}

		public GameState ProcessTick (GameState state)
		{
			// Step 0 - Determine how many people our income supports
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
			
			// Step 3c If less than 1 person round "up/down" to prevent very small changes from taking forever
			if (growthRate < 0)
				growthRate = Math.Min (growthRate, -1);
			else
				growthRate = Math.Max (growthRate, 1);
			
			// Step 3d If growing, don't grow over our effectice cap because then we'll just starve later 
			if (growthRate > 0 && state.Population + growthRate > effectivePopCap)
				return state;

			// Step 4 Grow!
			return GrowAtRate (state, growthRate, effectivePopCap);
		}

		GameState ConsumeResources (GameState state, ImmutableDictionary<string, double> consumedResources)
		{
			var currentResources = state.Resources.ToBuilder ();
			currentResources.Subtract (consumedResources);
			
			// Don't go negative when consuming population resources
			foreach (var resource in consumedResources.Keys.Where (x => currentResources[x] < 0).ToList ())
				currentResources [resource] = 0;

			return state.WithResources (currentResources);
		}

		public double FindEffectiveCap (GameState state)
		{
			var resourcesPerTick = ResourceEngine.CalculateAdditionalNextTick (state, 1.0);

			if (resourcesPerTick.HasMoreThan (GetRequirementsForPopulation (state.PopulationCap)))
				return state.PopulationCap;
			else
				return FindEffectivePopCap (state, resourcesPerTick);
		}
		
		public bool IsPopulationStarving (GameState state) 
		{
			var neededResource = GetRequirementsForPopulation (state);
			var nextTickResources = state.Resources.ToBuilder ();
			nextTickResources.Add (ResourceEngine.CalculateAdditionalNextTick (state, 1.0));
			return !nextTickResources.ToImmutable ().HasMoreThan (neededResource);
		}

		string FindMostMissingResource (GameState state, ImmutableDictionary<string, double> resourcesPerTick)
		{
			var delta = resourcesPerTick.ToBuilder ();
			delta.Subtract (GetRequirementsForPopulation (state));
			foreach (var need in PopNeed)
				delta[need.Key] = delta.AmountOf (need.Key) / need.Value;
			return delta.OrderBy (x => x.Value).Select (x => x.Key).Where (x => PopNeedNames.Contains (x)).First ();
		}

		double FindEffectivePopCap (GameState state, ImmutableDictionary<string, double> resourcesPerTick)
		{
			string mostMissingResource = FindMostMissingResource (state, resourcesPerTick);

			var delta = resourcesPerTick.ToBuilder ();
			delta.Subtract (GetRequirementsForPopulation (state));
			double peopleShort = delta.AmountOf (mostMissingResource) / PopNeed.AmountOf (mostMissingResource);
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

		public int GetPopUnitsForTotalPopulation (double population)
		{
			if (population < 1000) {
				return (int)Math.Floor (population / 100);
			} else if (population < 2000) {
				return 10 + (int)Math.Floor ((population - 1000) / 200);
			} else if (population < 4000) {
				return 15 + (int)Math.Floor ((population - 2000) / 500);
			} else if (population < 10000) {
				return 19 + (int)Math.Floor ((population - 4000) / 1000);
			} else if (population < 50000) {
				return 25 + (int)Math.Floor ((population - 10000) / 5000);
			} else if (population< 100000) {
				return 32 + (int)Math.Floor ((population - 50000) / 10000);
			} else {
				return 37 + (int)Math.Floor ((population - 100000) / 50000);
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
			const double R = .05;
			return R * ((popCap - popSize) / popSize) * popSize;
		}

		public double GetPopulationEfficiency (GameState state)
		{
			int buildingCount = state.AllBuildings ().Count ();
			int totalPopCount = GetPopUnitsForTotalPopulation (state.Population);
			return GetPopulationEfficiency (buildingCount, totalPopCount);
		}

		public double GetPopulationEfficiency (int buildingCount, int totalPopCount)
		{
			if (totalPopCount >= buildingCount)
				return 1.0;
			else
				return 1.0 - ((double)(buildingCount - totalPopCount) / (double)buildingCount);
		}
	}
}
