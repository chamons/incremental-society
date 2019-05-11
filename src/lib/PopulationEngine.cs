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
		ImmutableDictionary<string, double> PopNeed;
		double PopMin;
		YieldCache Yields;

		public PopulationEngine (JsonLoader json)
		{
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

		GameState GrowAtRate (GameState state, double rate, double cap) => state.WithPopulation (MathUtilities.Clamp (state.Population + rate, PopMin, cap));

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
