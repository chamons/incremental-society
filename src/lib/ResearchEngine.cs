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
	public class ResearchEngine
	{
		ResourceEngine ResourceEngine;
		JsonLoader Json;
		YieldCache Yields;

		public ResearchEngine (ResourceEngine resourceEngine, JsonLoader json)
		{
			ResourceEngine = resourceEngine;
			Json = json;
			Yields = new YieldCache (ResourceEngine.ResourceConfig);
		}

		ResearchDeclaration FindResearch (string techName) => Json.Research.Research.First (x => x.Name == techName);

		public bool CanResearch (GameState state, string techName)
		{
			if (state.ResearchUnlocks.Contains (techName))
				return false;

			var research = FindResearch (techName);
			if (research.Dependencies.AsNotNull ().Except (state.ResearchUnlocks).Any ())
				return false;

			if (!state.Resources.HasMoreThan (Yields.Total (research.Cost)))
				return false;

			return true;
		}

		public GameState Research (GameState state, string techName)
		{
			if (!CanResearch (state, techName))
				throw new InvalidOperationException ($"Tried to research {techName} but CanResearch returned false?");

			var research = FindResearch (techName);
			if (research.Cost != null)
				state = state.WithResources (state.Resources.Subtract (Yields.Total (research.Cost)));

			var unlocks = state.ResearchUnlocks.ToBuilder ();
			unlocks.Add (techName);
			return state.WithResearchUnlocks (unlocks);
		}
	}
}
