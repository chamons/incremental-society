using System;
using System.Collections.Generic;
using System.Linq;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

namespace IncrementalSociety
{
	public class EdictsEngine
	{
		Dictionary<string, EdictDeclaration> Edicts;
		ResourceEngine ResourceEngine;
		public EdictCooldownConfig EdictConfig;

		public EdictsEngine (ResourceEngine resourceEngine, JsonLoader json)
		{
			ResourceEngine = resourceEngine;
			EdictConfig = new EdictCooldownConfig (json.Edicts.Edicts.Select (x => x.Name));
			Edicts = json.Edicts.Edicts.ToDictionary (x => x.Name, x => x);
		}

		public Resources GetEdictCost (GameState state, string name) => ResourceEngine.GetResourcesBasedOnTech (state, Edicts[name].Cost);
		public Resources GetEdictCost (GameState state, EdictDeclaration edict) => ResourceEngine.GetResourcesBasedOnTech (state, edict.Cost);

		public GameState ProcessTick (GameState state)
		{
			if (state.Edicts.IsEmpty)
				return state;
			return state.WithEdicts (state.Edicts.Tick ());
		}

		public GameState ApplyEdict (GameState state, string name)
		{
			if (!CanApplyEdict (state, name))
				throw new InvalidOperationException ($"Attempted to apply edict {name} but unable to apply");

			var edict = Edicts[name];
			var resources = state.Resources.ToBuilder ();
			resources.Subtract (ResourceEngine.GetResourcesBasedOnTech (state, edict.Cost));
			resources.Add (ResourceEngine.GetResourcesBasedOnTech (state, edict.Provides));
			state = state.WithResources (resources.ToResources ());

			if (edict.Cooldown > 0)
				state = state.WithEdicts (state.Edicts.Add (edict.Name, edict.Cooldown));

			return state;
		}

		public bool CanApplyEdict (GameState state, string name)
		{
			if (!HasValidRequirements (state, name))
				return false;

			if (!CanApplyNow (state, name))
				return false;

			return true;
		}

		bool CanApplyNow (GameState state, string name)
		{
			if (state.Edicts[name] > 0)
				return false;

			var edict = Edicts[name];
			if (!state.Resources.HasMoreThan (GetEdictCost (state, edict)))
				return false;
			return true;
		}

		bool HasValidRequirements (GameState state, string name)
		{
			var edict = Edicts[name];

			if (!state.HasResearch (edict.RequireTechnology))
				return false;
			if (edict.RequireBuilding != null && !state.AllBuildings ().Any (x => x == edict.RequireBuilding))
				return false;
			return true;
		}

		public IEnumerable<(string Name, bool CanApply)> AvailableEdicts (GameState state)
		{
			return Edicts.Where (x => HasValidRequirements (state, x.Key)).Select (x => (x.Key, CanApplyNow (state, x.Key)));
		}
	}
}