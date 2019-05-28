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

			return state;
		}

		public bool CanApplyEdict (GameState state, string name)
		{
			if (state.Edicts[name] > 0)
				return false;

			var edict = Edicts[name];
			if (!state.Resources.HasMoreThan (GetEdictCost (state, edict)))
				return false;

			if (!state.HasResearch (edict.RequireTechnology))
				return false;

			if (edict.RequireBuilding != null && !state.AllBuildings ().Any (x => x == edict.RequireBuilding))
				return false;

			return true;
		}

		public IEnumerable<(string Name, int Cooldown)> AvailableEdicts (GameState state)
		{
			return null;
		}
	}
}