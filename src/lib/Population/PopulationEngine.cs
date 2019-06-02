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
		double PopMin;

		ResourceConfig ResourceConfig => ResourceEngine.ResourceConfig;

		public PopulationEngine (ResourceEngine resourceEngine, PopulationCapacity populationCapacity, PopulationResources populationResourceFinder, double popMin)
		{
			ResourceEngine = resourceEngine;
			PopulationCapacity = populationCapacity;
			PopulationResources = populationResourceFinder;
			PopulationGrowthCurve = new PopulationGrowthCurve ();
			PopMin = popMin;
		}

		public GameState ProcessTick (GameState state)
		{

			var neededResource = PopulationResources.GetRequirementsForCurrentPopulation (state);

			bool starved = !state.Resources.HasMoreThan (neededResource);
			state = ConsumeResources (state, neededResource);

			double effectivePopCap = PopulationCapacity.FindEffectiveCap (state);
			double growthRate = PopulationGrowthCurve.GetGrowthRate (state, starved, effectivePopCap);

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

		GameState GrowAtRate (GameState state, double rate, double cap)
		{
			double newPopulation;
			if (rate > 0)
				newPopulation = Math.Min (state.Population + rate, cap);
			else
				newPopulation = Math.Max (state.Population + rate, PopMin);

			return state.WithPopulation (newPopulation);
		}
	}
}