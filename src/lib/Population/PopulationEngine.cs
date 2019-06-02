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
	public class PopulationEngine
	{
		ResourceEngine ResourceEngine;
		PopulationCapacity PopulationCapacity;
		PopulationResources PopulationResources;
		PopulationGrowthCurve PopulationGrowthCurve;
		PopulationNeeds PopulationNeeds;
		double PopMin;

		ResourceConfig ResourceConfig => ResourceEngine.ResourceConfig;

		public PopulationEngine (ResourceEngine resourceEngine, PopulationCapacity populationCapacity, PopulationResources populationResourceFinder, JsonLoader loader)
		{
			PopMin = loader.Game.MinPopulation;

			ResourceEngine = resourceEngine;
			PopulationCapacity = populationCapacity;
			PopulationResources = populationResourceFinder;
			PopulationGrowthCurve = new PopulationGrowthCurve (PopulationCapacity, PopMin);
			PopulationNeeds = new PopulationNeeds (ResourceEngine, loader, PopulationCapacity, PopulationResources);
		}

		public GameState ProcessTick (GameState state)
		{
			var neededResource = PopulationResources.GetRequirementsForCurrentPopulation (state);
			var happiness = PopulationNeeds.CalculateHappiness (state);
			var health = PopulationNeeds.CalculateHealth (state);

			double growthRate = PopulationGrowthCurve.GetGrowthRate (state, happiness, health);

			state = PopulationNeeds.ConsumeResources (state);
			return state.WithPopulation (state.Population + growthRate);
		}
	}
}