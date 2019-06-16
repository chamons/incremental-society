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
	public class PopulationResources
	{
		ResourceEngine ResourceEngine;
		PopulationBuildingInfo PopulationBuildingInfo;
		JsonLoader Json;
		List<string> LuxuryNeedsNames;

		public PopulationResources (ResourceEngine resourceEngine, PopulationBuildingInfo populationBuildingInfo, JsonLoader json)
		{
			ResourceEngine = resourceEngine;
			PopulationBuildingInfo = populationBuildingInfo;
			Json = json;
			LuxuryNeedsNames = Json.Game.LuxuryPopulationNeeds.Select (x => x.Name).ToList ();
		}

		public Resources GetRequirementsPerPop (GameState state)
		{
			return ResourceEngine.GetResourcesBasedOnTech (state, Json.Game.PopulationNeeds);
		}

		public Resources GetRequirementsForCurrentPopulation (GameState state) => GetRequirementsForPopulation (state, state.Population);

		public Resources GetRequirementsForPopulation (GameState state, double population)
		{
			return GetRequirementsPerPop (state).Multiply (population);
		}

		public Resources GetLuxuryPerPop (GameState state)
		{
			return ResourceEngine.GetResourcesBasedOnTech (state, Json.Game.LuxuryPopulationNeeds);
		}

		public Resources GetLuxuryForCurrentPopulation (GameState state) => GetLuxuryForCurrentPopulation (state, state.Population);

		public Resources GetLuxuryForCurrentPopulation (GameState state, double population)
		{
			return GetLuxuryPerPop (state).Multiply (population);
		}

		public double FindResourceEffectivePopCap (GameState state, Resources resourcesPerTick)
		{
			string mostMissingResource = FindMostMissingResource (state, resourcesPerTick);
			var totalPopNeed = GetRequirementsForCurrentPopulation (state);
			var needsPerPop = GetRequirementsPerPop (state);
			var delta = resourcesPerTick.ToBuilder ();
			delta.Subtract (totalPopNeed);
			double peopleShort = delta[mostMissingResource] / needsPerPop[mostMissingResource];
			return state.Population + peopleShort;
		}

		string FindMostMissingResource (GameState state, Resources resourcesPerTick)
		{
			var needsPerPop = GetRequirementsPerPop (state);

			var delta = resourcesPerTick.Subtract (GetRequirementsForCurrentPopulation (state)).ToBuilder ();
			foreach (var need in needsPerPop)
				delta[need.ResourceName] = delta[need.ResourceName] / need.Value;
			return delta.OrderBy (x => x.Value).Select (x => x.ResourceName).Where (x => needsPerPop[x] > 0).First ();
		}

		Resources.Builder ResourcesWithNextTick (GameState state)
		{
			double efficiency = PopulationBuildingInfo.GetPopulationEfficiency (state);
			var nextTickResources = state.Resources.ToBuilder ();
			nextTickResources.Add (ResourceEngine.CalculateAdditionalNextTick (state, efficiency));
			return nextTickResources;
		}

		public bool IsPopulationStarving (GameState state)
		{
			var neededResource = GetRequirementsForCurrentPopulation (state);
			var nextTickResources = ResourcesWithNextTick (state);
			return !nextTickResources.HasMoreThan (neededResource);
		}

		public IEnumerable<double> FindLuxuryRatios (GameState state)
		{
			double [] ratios = new double [LuxuryNeedsNames.Count];

			var luxuryResource = GetLuxuryForCurrentPopulation (state);
			var nextTickResources = ResourcesWithNextTick (state);
			for (int i = 0; i < LuxuryNeedsNames.Count; ++i) {
				string luxuryName = LuxuryNeedsNames[i];
				ratios [i] = MathUtilities.Clamp (nextTickResources[luxuryName] / luxuryResource[luxuryName], 0, 1);
			}
			return ratios;
		}
	}
}