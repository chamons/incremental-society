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

			PopMin = json.Game.MinPopulation;
		}

		public ImmutableDictionary<string, double> GetFullRequirementsForNextTick (GameState state)
		{
			var amount = PopNeed.ToBuilder ();
			amount.Multiply (state.Population);
			return amount.ToImmutable ();
		}

		public GameState ProcessTick (GameState state)
		{
			if (state.Resources.HasMoreThan (GetFullRequirementsForNextTick (state))) {
				return GrowAtRate (state, GetGrowthRate (state.Population, state.PopulationCap), state.PopulationCap);
			}
			else {
				double effectivePopCap = FindEffectivePopCap (state);
				return GrowAtRate (state, GetGrowthRate (state.Population, effectivePopCap), state.PopulationCap);
			}
		}

		string FindMostMissingResource (GameState state)
		{
			var delta = state.Resources.ToBuilder ();
			delta.Subtract (GetFullRequirementsForNextTick (state));
			foreach (var need in PopNeed)
				delta[need.Key] = delta.AmountOf (need.Key) / need.Value;
			return delta.OrderBy (x => x.Value).Select (x => x.Key).First ();
		}

		double FindEffectivePopCap (GameState state)
		{
			string mostMissingResource = FindMostMissingResource (state);

			var delta = state.Resources.ToBuilder ();
			delta.Subtract (GetFullRequirementsForNextTick (state));
			double peopleShort = delta.AmountOf (mostMissingResource) / PopNeed.AmountOf (mostMissingResource);
			return state.Population + peopleShort;
		}

		GameState GrowAtRate (GameState state, double rate, double cap)
		{
			if (rate > 0)
				return state.WithPopulation (Math.Min (state.Population + rate, cap));
			else
				return state.WithPopulation (Math.Max (state.Population + rate, PopMin));
		}

		public double GetHousingCapacity (GameState state) => state.AllBuildings ().Sum (x => ResourceEngine.FindBuilding (x).HousingCapacity);

		public bool CanIncreasePopulationCap (GameState state)
		{
			return GetHousingCapacity (state) >= GetNextPopBreakpoint (state.PopulationCap);
		}

		public GameState IncreasePopulationCap (GameState state)
		{
			if (!CanIncreasePopulationCap (state))
				throw new InvalidOperationException ($"Unable to decrease pop cap {state.PopulationCap}");
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
