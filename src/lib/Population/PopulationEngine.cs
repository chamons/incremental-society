using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Text;
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

		public PopulationEngine (ResourceEngine resourceEngine, PopulationCapacity populationCapacity, PopulationResources populationResourceFinder, PopulationNeeds populationNeeds, double popMin)
		{
			PopMin = popMin;

			ResourceEngine = resourceEngine;
			PopulationCapacity = populationCapacity;
			PopulationResources = populationResourceFinder;
			PopulationNeeds = populationNeeds;
			PopulationGrowthCurve = new PopulationGrowthCurve (PopulationCapacity, PopMin);
		}

		public GameState ProcessTick (GameState state)
		{
			double growthRate = CalculateGrowthRate (state);

			state = PopulationNeeds.ConsumeResources (state);
			return state.WithPopulation (state.Population + growthRate);
		}

		public double CalculateGrowthRate (GameState state)
		{
			var neededResource = PopulationResources.GetRequirementsForCurrentPopulation (state);
			var happiness = PopulationNeeds.CalculateHappiness (state);
			var health = PopulationNeeds.CalculateHealth (state);

			return PopulationGrowthCurve.GetGrowthRate (state, happiness, health);
		}
	}
}