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
		
		public int RegionCapacity => Json.Game.RegionCapacity;
		
		public ResourceEngine (JsonLoader json)
		{
			Json = json;
			Yields = new YieldCache ();
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
			var activeConversions = new List<(string Conversion, double LeastAmount)> ();
			do {
				// Determine next tick
				var tickOfResources = CalculateAdditionalNextTick (state, efficiency);
				var newResources = state.Resources.ToBuilder ();
				newResources.Add (tickOfResources);

				// If we're all positive, then go with that
				if (!newResources.Keys.Any(x => newResources[x] < 0))
					return state.WithResources (newResources.ToImmutable ());
			
				// Find the largets negative resource
				var leastResource = newResources.OrderBy (x => x.Value).First().Key;

				// Find the best conversion of that type that is enabled
				activeConversions.Clear ();
				foreach (var building in state.AllBuildings ())
					foreach (var conversion in GetBuildingConversionResources (building).Where (x => IsConversionEnabled (state, x.Name)))
						activeConversions.Add ((conversion.Name, conversion.Resources.AmountOf (leastResource)));

				string bestConversion = activeConversions.OrderBy (x => x.LeastAmount).First ().Conversion;

				// Disable that conversion
				state = state.WithDisabledConversions (state.DisabledConversions.Add (bestConversion));
			}
			while (activeConversions.Count > 0);
			
			return state;
		}

		public ImmutableDictionary<string, double> CalculateAdditionalNextTick (GameState state, double efficiency)
		{
			var additional = ImmutableDictionary.CreateBuilder<string, double> ();
			foreach (var building in state.AllBuildings ()) {
				additional.Add (GetBuildingResources (building));

				var conversions = GetBuildingConversionResources (building);
				foreach (var conversion in conversions.Where (x => IsConversionEnabled (state, x.Name)))
					additional.Add (conversion.Resources);
			}
			additional.Multiply (efficiency);
			return additional.ToImmutable ();
		}

		public ImmutableDictionary<string, double> GetBuildingResources (string name)
		{
			var building = FindBuilding (name);
			return Yields.Total (building.Yield);
		}

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
	}
}
