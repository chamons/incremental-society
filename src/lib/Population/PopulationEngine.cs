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
			// Step 0 - Determine how many people our income and housing supports
			double effectivePopCap = PopulationCapacity.FindEffectiveCap (state);
#if DEBUG
			if (!effectivePopCap.HasValue())
				throw new InvalidOperationException ($"Processing population tick produced invalid population cap: {effectivePopCap}");
#endif

			// Step 1 - Determine how many resources we need for our current population
			var neededResource = PopulationResources.GetRequirementsForCurrentPopulation (state);

			// Step 2 - Consume if we have enough, else consume as much as we can
			bool starved = !state.Resources.HasMoreThan (neededResource);
			state = ConsumeResources (state, neededResource);

			// Step 3a Get new desired growth rate
			double growthRate = PopulationGrowthCurve.GetGrowthRate (state.Population, effectivePopCap);

			// Step 3b If we starved some people, multiple negative by x5
			if (starved)
				growthRate *= 5;

			// Step 3c Tweak the growth rate to be "nicer":
			// - If we're within one of the cap, round our rate to make a nice .25
			// - Else if our rate is less than one, round "up/down" to prevent very small changes from taking forever
			const double MinGrowth = 0.25;
			if (growthRate < 0) {
				if (state.Population - effectivePopCap < MinGrowth)
					growthRate = effectivePopCap - state.Population;
				else
					growthRate = Math.Min (growthRate, -MinGrowth);
			}
			else {
				if (effectivePopCap - state.Population < MinGrowth)
					growthRate = effectivePopCap - state.Population;
				else
					growthRate = Math.Max (growthRate, MinGrowth);
			}

			// Step 4 Grow!
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