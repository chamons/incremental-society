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
		PopUnits PopUnits;
		double HappinessGainPerFullLuxury;
		double HappinessLossPerLuxuryMissing;
		double HappinessLossStaring;
		double HappinessLossPerExtraPop;
		double HealthLossStaring;
		double HealthLossPerExtraPop;

		ResourceConfig ResourceConfig => ResourceEngine.ResourceConfig;

		public PopulationNeeds (ResourceEngine resourceEngine, JsonLoader json, PopUnits popUnits, PopulationResources populationResources)
		{
			ResourceEngine = resourceEngine;
			PopulationResources = populationResources;
			PopUnits = popUnits;
			HappinessGainPerFullLuxury = json.Game.HappinessGainPerFullLuxury;
			HappinessLossPerLuxuryMissing = json.Game.HappinessLossPerLuxuryMissing;
			HappinessLossStaring = json.Game.HappinessLossStaring;
			HappinessLossPerExtraPop = json.Game.HappinessLossPerExtraPop;
			HealthLossStaring = json.Game.HealthLossStaring;
			HealthLossPerExtraPop = json.Game.HealthLossPerExtraPop;
		}

		public PopulationRatio CalculateHappiness (double population, IEnumerable <double> luxuryGoods, bool starving)
		{
			// No one should be happy when people are actively starving in your country
			if (starving)
				return PopulationRatio.Create (0);

			double happiness = 1.0;

			double currentPops = PopUnits.GetPopUnitsForTotalPopulation (population);
			double popsOver = currentPops - HappinessLossStaring;
			if (popsOver > 0)
				happiness -= popsOver * HappinessLossPerExtraPop;

			foreach (var luxRatio in luxuryGoods) {
				if (luxRatio >= 1)
					happiness += HappinessGainPerFullLuxury;
				else
					happiness -= HappinessLossPerLuxuryMissing * (1 - luxRatio);
			}

			return PopulationRatio.Create (MathUtilities.Clamp (happiness, 0, 1));
		}

		public PopulationRatio CalculateHappiness (GameState state)
		{
			return CalculateHappiness (state.Population, PopulationResources.FindLuxuryRatios (state), PopulationResources.IsPopulationStarving (state));
		}

		public PopulationRatio CalculateHealth (double population)
		{
			double health = 1.0;

			double currentPops = PopUnits.GetPopUnitsForTotalPopulation (population);
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