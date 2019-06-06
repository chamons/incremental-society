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
		JsonLoader Json;
		List<string> LuxaryNeedsNames;

		public PopulationResources (ResourceEngine resourceEngine, JsonLoader json)
		{
			ResourceEngine = resourceEngine;
			Json = json;
			LuxaryNeedsNames = Json.Game.LuxaryPopulationNeeds.Select (x => x.Name).ToList ();
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

		public Resources GetLuxaryPerPop (GameState state)
		{
			return ResourceEngine.GetResourcesBasedOnTech (state, Json.Game.LuxaryPopulationNeeds);
		}

		public Resources GetLuxaryForCurrentPopulation (GameState state) => GetLuxaryForCurrentPopulation (state, state.Population);

		public Resources GetLuxaryForCurrentPopulation (GameState state, double population)
		{
			return GetLuxaryPerPop (state).Multiply (population);
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
			var nextTickResources = state.Resources.ToBuilder ();
			// TODO - 1.0 here is wrong
			nextTickResources.Add (ResourceEngine.CalculateAdditionalNextTick (state, 1.0));
			return nextTickResources;
		}

		public bool IsPopulationStarving (GameState state)
		{
			var neededResource = GetRequirementsForCurrentPopulation (state);
			var nextTickResources = ResourcesWithNextTick (state);
			return !nextTickResources.HasMoreThan (neededResource);
		}

		public IEnumerable<double> FindLuxaryRatios (GameState state)
		{
			double [] ratios = new double [LuxaryNeedsNames.Count];

			var luxaryResource = GetLuxaryForCurrentPopulation (state);
			var nextTickResources = ResourcesWithNextTick (state);
			for (int i = 0; i < LuxaryNeedsNames.Count; ++i) {
				string luxaryName = LuxaryNeedsNames[i];
				ratios [i] = MathUtilities.Clamp (nextTickResources[luxaryName] / luxaryResource[luxaryName], 0, 1);
			}
			return ratios;
		}
	}
}