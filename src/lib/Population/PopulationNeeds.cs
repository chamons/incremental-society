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
			HappinessLossPerLuxaryMissing = json.Game.HappinessLossPerLuxaryMissing;
			HappinessLossStaring = json.Game.HappinessLossStaring;
			HappinessLossPerExtraPop = json.Game.HappinessLossPerExtraPop;
			HealthLossStaring = json.Game.HealthLossStaring;
			HealthLossPerExtraPop = json.Game.HealthLossPerExtraPop;
		}

		public PopulationRatio CalculateHappiness (double population, double luxaryGoodRaio, double totalLuxaryGoods)
		{
			return PopulationRatio.Create (1);
		}

		public PopulationRatio CalculateHappiness (GameState state)
		{
			// TODO - Fix
			return CalculateHappiness (state.Population, 1, 1);
		}

		public PopulationRatio CalculateHealth (GameState state)
		{
			return CalculateHealth (state.Population);
		}

		public PopulationRatio CalculateHealth (double population)
		{
			return PopulationRatio.Create (1);
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