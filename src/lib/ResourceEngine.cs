using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

namespace IncrementalSociety
{
	public class ResourceEngine
	{
		JsonLoader Json;
		YieldCache Yields;

		public ResourceConfig ResourceConfig;

		public int RegionCapacity => Json.Game.RegionCapacity;

		public ResourceEngine (JsonLoader json)
		{
			Json = json;
			ResourceConfig = new ResourceConfig (json.Resources.Resources.Select (x => x.Name));
			Yields = new YieldCache (ResourceConfig);
		}

		public IEnumerable<Building> Buildings => Json.Buildings.Buildings;

		public Building FindBuilding (string name)
		{
			var building = Json.Buildings.Buildings.FirstOrDefault (x => x.Name == name);
			if (building == null)
				throw new InvalidOperationException ($"Unable to find building \"{name}\" in resources");
			return building;
		}

		public GameState AddTickOfResources (GameState state, double efficiency)
		{
			Resources.Builder newResources = null;
			do {
				// Determine next tick
				var tickOfResources = CalculateAdditionalNextTick (state, efficiency);
				newResources = state.Resources.ToBuilder ();
				newResources.Add (tickOfResources);

				// If we're all positive, then go with that
				if (newResources.All (x => x.Value > 0))
					break;

				string conversion = FindConversionToDisable (state, newResources.ToResources ());

				// Disable that conversion if found, else break;
				if (conversion != null)
					state = state.WithDisabledConversions (state.DisabledConversions.Add (conversion));
				else
					break;
			}
			while (true);

			return state.WithResources (newResources.ToResources ());
		}

		public GameState ConstrainResourcesToStorage (GameState state)
		{
			Resources.Builder resources = state.Resources.ToBuilder ();
			Resources resourceStorage = GetResourceStorage (state);

			foreach (var resource in ResourceConfig.ResourceNames) {
				double storage = resourceStorage[resource];
				if (resources[resource] > storage)
					resources[resource] = storage;
			}
			return state.WithResources (resources);
		}

		string FindConversionToDisable (GameState state, Resources newResources)
		{
			foreach (string missingResource in newResources.Where (x => x.Value < 0).Select (x => x.ResourceName).OrderBy (x => x)) {
				var conversions = GetConversions (state, missingResource);
				if (conversions.Count <= 0)
					break;

				return conversions.OrderBy (x => x.Amount).Select (x => x.Conversion).First();
			}
			return null;
		}

		List<(string Conversion, double Amount)> GetConversions (GameState state, string missingResource)
		{
				var activeConversions = new List<(string Conversion, double Amount)> ();
				foreach (var building in state.AllBuildings ())
					foreach (var conversion in GetBuildingConversionResources (building).Where (x => IsConversionEnabled (state, x.Name)))
						if (conversion.Resources[missingResource] < 0)
							activeConversions.Add ((conversion.Name, conversion.Resources[missingResource]));
				return activeConversions;
		}

		public Resources CalculateAdditionalNextTick (GameState state, double efficiency)
		{
			var additional = ResourceConfig.CreateBuilder ();
			foreach (var building in state.AllBuildings ()) {
				additional.AddWithMultiply (GetBuildingResources (building), efficiency);

				var conversions = GetBuildingConversionResources (building);
				foreach (var conversion in conversions.Where (x => IsConversionEnabled (state, x.Name)))
					additional.Add (conversion.Resources);
			}
			return additional.ToResources ();
		}

		public Resources GetBuildingResources (string name) => Yields.Total (FindBuilding (name).Yield);
		public Resources GetBuildingCost (string name) => Yields.Total (FindBuilding (name).Cost);
		public Resources GetBuildingStorage (string name) => Yields.Total (FindBuilding (name).Storage);

		public List<(string Name, Resources Resources)> GetBuildingConversionResources (string name)
		{
			var conversion = new List<(string name, Resources resources)> ();
			var building = FindBuilding (name);
			foreach (var conversionYield in building.ConversionYield.AsNotNull ())
				conversion.Add ((conversionYield.Name, Yields.From (conversionYield)));
			return conversion;
		}

		public bool IsConversionEnabled (GameState state, string name) => !state.DisabledConversions.Contains (name);

		public List<(string Name, bool Enabled)> GetConversions (GameState state)
		{
			var consideredConversions = new HashSet<string> ();
			var allConversions = new List<(string Conversion, bool Enabled)> ();
			foreach (var building in state.AllBuildings()) {
				foreach (var conversion in GetBuildingConversionResources (building)) {
					if (!consideredConversions.Contains (conversion.Name)) {
						consideredConversions.Add (conversion.Name);
						allConversions.Add ((conversion.Name, IsConversionEnabled (state, conversion.Name)));
					}
				}
			}
			return allConversions;
		}

		public Resources GetResourceStorage (GameState state)
		{
			var storage = ResourceConfig.CreateBuilder ();
			foreach (var yields in state.AllBuildings ().Select (x => FindBuilding (x).Storage))
				storage.Add (Yields.Total (yields));
			return storage.ToResources ();
		}
	}
}
