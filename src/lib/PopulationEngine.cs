using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Text;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

namespace IncrementalSociety
{
	public class PopulationEngine
	{
		ImmutableDictionary<string, double> PopNeed;
		YieldCache Yields;

		public PopulationEngine (JsonLoader json)
		{
			Yields = new YieldCache ();
			PopNeed = CalculatePopNeed (json);
		}

		ImmutableDictionary<string, double> CalculatePopNeed (JsonLoader json)
		{
			var totalNeed = ImmutableDictionary.CreateBuilder<string, double> ();
			foreach (var need in json.Game.PopulationNeeds)
				totalNeed.Add (Yields.Total (need.Resource));
			return totalNeed.ToImmutable ();
		}

		public ImmutableDictionary<string, double> GetFullRequirementsForNextTick (GameState state)
		{
			var amount = PopNeed.ToBuilder ();
			amount.Multiply (state.Population);
			return amount.ToImmutable ();
		}

		public int GetPopUnitsForTotalPopulation (double population)
		{
			if (population < 10000) {
				return (int)Math.Round (population / 1000);
			} else if (population < 20000) {
				return 10 + (int)Math.Round ((population - 10000) / 2000);
			} else if (population < 40000) {
				return 15 + (int)Math.Round ((population - 20000) / 5000);
			} else if (population < 100000) {
				return 19 + (int)Math.Round ((population - 40000) / 10000);
			} else {
				return 25 + (int)Math.Round ((population - 100000) / 100000);
			}
		}

		public static double GetGrowthRate (double popSize, double popCap)
		{
			// Logistic growth
			const double R = .05;
			return R * ((popCap - popSize) / popSize) * popSize;
		}
	}
}
