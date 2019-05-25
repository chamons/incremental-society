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

		ResourceConfig ResourceConfig => ResourceEngine.ResourceConfig;

		public ResearchEngine (ResourceEngine resourceEngine, JsonLoader json)
		{
			ResourceEngine = resourceEngine;
			Json = json;
		}

		ResearchDeclaration FindResearch (string techName) => Json.Research.Research.First (x => x.Name == techName);

		bool StateHasUnlocked (GameState state, string techName) => state.ResearchUnlocks.AsNotNull ().Contains (techName);

		public bool CanResearch (GameState state, string techName)
		{
			if (StateHasUnlocked (state, techName))
				return false;

			var research = FindResearch (techName);
			if (research.Dependencies.AsNotNull ().Except (state.ResearchUnlocks).Any ())
				return false;

			if (!state.Resources.HasMoreThan (ResourceConfig.Create (research.Cost)))
				return false;

			return true;
		}

		public GameState Research (GameState state, string techName)
		{
			if (!CanResearch (state, techName))
				throw new InvalidOperationException ($"Tried to research {techName} but CanResearch returned false?");

			var research = FindResearch (techName);
			if (research.Cost != null)
				state = state.WithResources (state.Resources.Subtract (ResourceConfig.Create (research.Cost)));

			var unlocks = state.ResearchUnlocks.ToBuilder ();
			unlocks.Add (techName);
			return state.WithResearchUnlocks (unlocks);
		}

		public List<ResearchItem> GetCurrentResearchOptions (GameState state)
		{
			var availableResearch = Json.Research.Research.Where (x => {
					return StateHasUnlocked (state, x.Name) ||
							x.Dependencies.AsNotNull ().All (y => StateHasUnlocked (state, y));
				});
			return availableResearch.Select (x => new ResearchItem (x.Name, x.Description, StateHasUnlocked (state, x.Name), ResourceConfig.Create (x.Cost))).ToList ();
		}
	}
}
