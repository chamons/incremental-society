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

		void AssertValidSpecializationIndex (ResearchDeclaration research, int index)
		{
			int count = research.Specializations.AsNotNull ().Count ();
			if (count == 0) {
				if (index != -1)
					throw new InvalidOperationException ($"Tried to index {research.Name} with index {index} but does not have specializations?");
			}
			else {
				if (index < 0 || index > count)
					throw new InvalidOperationException ($"Tried to index {research.Name} with index {index} but invalid for length {count}?");
			}
		}

		public bool CanResearch (GameState state, string techName, int specialization = -1)
		{
			if (state.HasResearch (techName))
				return false;

			var research = FindResearch (techName);
			if (research.IsNotStandalone)
				return false;

			if (research.Dependencies.AsNotNull ().Except (state.ResearchUnlocks).Any ())
				return false;

			AssertValidSpecializationIndex (research, specialization);

			if (specialization != -1) {
				if (state.HasResearch (research.Specializations[specialization]))
					return false;
			}

			if (!state.Resources.HasMoreThan (ResourceConfig.Create (research.Cost)))
				return false;

			return true;
		}

		public GameState Research (GameState state, string techName, int specialization = -1)
		{
			if (!CanResearch (state, techName, specialization))
				throw new InvalidOperationException ($"Tried to research {techName} but CanResearch returned false?");

			var research = FindResearch (techName);
			if (research.Cost != null)
				state = state.WithResources (state.Resources.Subtract (ResourceConfig.Create (research.Cost)));

			var unlocks = state.ResearchUnlocks.ToBuilder ();
			unlocks.Add (techName);

			if (specialization != -1) {
				unlocks.Add (research.Specializations[specialization]);
			}

			return state.WithResearchUnlocks (unlocks);
		}

		public List<ResearchItem> GetCurrentResearchOptions (GameState state)
		{
			var availableResearch = Json.Research.Research.Where (x => {
					return !x.IsNotStandalone && !state.HasResearch (x.Name) && x.Dependencies.AsNotNull ().All (y => state.HasResearch (y));
				});
			return availableResearch.Select (x => new ResearchItem (x.Name, x.Description, ResourceConfig.Create (x.Cost))).ToList ();
		}

		public List<ResearchItem> GetResearchSpecializations (string techName)
		{
			var baseResearch = FindResearch (techName);
			return baseResearch.Specializations.AsNotNull ().Select (x =>
			{
				var specialization = FindResearch (x);
				return new ResearchItem (specialization.Name, specialization.Description, ResourceConfig.Create (baseResearch.Cost));
			}).ToList ();
		}

		
	}
}
