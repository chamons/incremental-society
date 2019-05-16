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
		HashSet<string> BasicResources;

		public int RegionCapacity => Json.Game.RegionCapacity;
		
		public ResourceEngine (JsonLoader json)
		{
			Json = json;
			Yields = new YieldCache ();
			BasicResources = new HashSet <string> (json.Resources.Resources.Where (x => x.Basic).Select (x => x.Name)); 
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
			ImmutableDictionary<string, double>.Builder newResources = null; 
			do {
				// Determine next tick
				var tickOfResources = CalculateAdditionalNextTick (state, efficiency);
				newResources = state.Resources.ToBuilder ();
				newResources.Add (tickOfResources);

				// If we're all positive, then go with that
				if (!newResources.Keys.Any(x => newResources[x] < 0))
					break;

				string conversion = FindConversionToDisable (state, newResources.ToImmutable ());

				// Disable that conversion if found, else break;
				if (conversion != null)
					state = state.WithDisabledConversions (state.DisabledConversions.Add (conversion));
				else
					break;
			}
			while (true);

			return state.WithResources (newResources.ToImmutable ());
		}

		public GameState ConstrainResourcesToStorage (GameState state)
		{
			ImmutableDictionary<string, double>.Builder resources = state.Resources.ToBuilder ();
			ImmutableDictionary<string, double> resourceStorage = GetResourceStorage (state);

			foreach (var resource in resources.Keys.ToList ()) {
				double storage = resourceStorage.AmountOf (resource);
				if (resources[resource] > storage)
					resources[resource] = storage;
			}
			return state.WithResources (resources);
		}

		string FindConversionToDisable (GameState state, ImmutableDictionary <string, double> newResources)
		{
			foreach (string missingResource in newResources.Where (x => x.Value < 0).Select (x => x.Key).OrderBy (x => x)) {
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
						if (conversion.Resources.AmountOf (missingResource) < 0)
							activeConversions.Add ((conversion.Name, conversion.Resources.AmountOf (missingResource)));
				return activeConversions;
		}

		public ImmutableDictionary<string, double> CalculateAdditionalNextTick (GameState state, double efficiency)
		{
			var additional = ImmutableDictionary.CreateBuilder<string, double> ();
			foreach (var building in state.AllBuildings ()) {
				additional.Add (GetBuildingResources (building));

				if (efficiency != 1) {
					foreach (var nonBasicResource in additional.Keys.Where (x => !BasicResources.Contains (x)).ToList ())
						additional[nonBasicResource] = additional[nonBasicResource] * efficiency;
				}

				var conversions = GetBuildingConversionResources (building);
				foreach (var conversion in conversions.Where (x => IsConversionEnabled (state, x.Name)))
					additional.Add (conversion.Resources);
			}
			return additional.ToImmutable ();
		}

		public ImmutableDictionary<string, double> GetBuildingResources (string name) => Yields.Total (FindBuilding (name).Yield);
		public ImmutableDictionary<string, double> GetBuildingCost (string name) => Yields.Total (FindBuilding (name).Cost);
		public ImmutableDictionary<string, double> GetBuildingStorage (string name) => Yields.Total (FindBuilding (name).Storage);

		public List<(string Name, ImmutableDictionary<string, double> Resources)> GetBuildingConversionResources (string name)
		{
			var conversion = new List<(string name, ImmutableDictionary<string, double> resources)> ();
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

		public ImmutableDictionary<string, double> GetResourceStorage (GameState state)
		{	
			var storage = ImmutableDictionary.CreateBuilder <string, double> ();
			foreach (var yields in state.AllBuildings ().Select (x => FindBuilding (x).Storage))
				storage.Add (Yields.Total (yields));
			return storage.ToImmutable ();
		}
	}
}
