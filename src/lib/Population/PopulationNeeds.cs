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
	public class PopulationNeeds
	{
		ResourceEngine ResourceEngine;
		PopulationResources PopulationResources;
		PopulationCapacity PopulationCapacity;
		double HappinessGainPerFullLuxary;
		double HappinessLossPerLuxaryMissing;
		double HappinessLossStaring;
		double HappinessLossPerExtraPop;
		double HealthLossStaring;
		double HealthLossPerExtraPop;

		ResourceConfig ResourceConfig => ResourceEngine.ResourceConfig;

		public PopulationNeeds (ResourceEngine resourceEngine, JsonLoader json, PopulationCapacity populationCapacity, PopulationResources populationResources)
		{
			ResourceEngine = resourceEngine;
			PopulationResources = populationResources;
			PopulationCapacity = populationCapacity;
			HappinessGainPerFullLuxary = json.Game.HappinessGainPerFullLuxary;
			HappinessLossPerLuxaryMissing = json.Game.HappinessLossPerLuxaryMissing;
			HappinessLossStaring = json.Game.HappinessLossStaring;
			HappinessLossPerExtraPop = json.Game.HappinessLossPerExtraPop;
			HealthLossStaring = json.Game.HealthLossStaring;
			HealthLossPerExtraPop = json.Game.HealthLossPerExtraPop;
		}

		public PopulationRatio CalculateHappiness (double population, IEnumerable <double> luxaryGoods, bool starving)
		{
			// No one should be happy when people are actively starving in your country
			if (starving)
				return PopulationRatio.Create (0);

			double happiness = 1.0;

			double currentPops = PopulationCapacity.GetPopUnitsForTotalPopulation (population);
			double popsOver = currentPops - HappinessLossStaring;
			if (popsOver > 0)
				happiness -= popsOver * HappinessLossPerExtraPop;

			foreach (var luxRatio in luxaryGoods) {
				if (luxRatio >= 1)
					happiness += HappinessGainPerFullLuxary;
				else
					happiness -= HappinessLossPerLuxaryMissing * (1 - luxRatio);
			}

			return PopulationRatio.Create (MathUtilities.Clamp (happiness, 0, 1));
		}

		public PopulationRatio CalculateHappiness (GameState state)
		{
			// TODO - Fix
			return CalculateHappiness (state.Population, Enumerable.Empty<double> (), false);
		}

		public PopulationRatio CalculateHealth (double population)
		{
			double health = 1.0;

			double currentPops = PopulationCapacity.GetPopUnitsForTotalPopulation (population);
			double popsOver = currentPops - HealthLossStaring;

			if (popsOver > 0)
				health -= popsOver * HealthLossPerExtraPop;

			return PopulationRatio.Create (MathUtilities.Clamp (health, 0, 1));
		}

		public PopulationRatio CalculateHealth (GameState state)
		{
			return CalculateHealth (state.Population);
		}

		public GameState ConsumeResources (GameState state)
		{
			var consumedResources = PopulationResources.GetRequirementsForCurrentPopulation (state);

			var currentResources = state.Resources.ToBuilder ();
			currentResources.Subtract (consumedResources);
			currentResources.FloorAtZero ();
			return state.WithResources (currentResources);
		}
	}
}