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
		
		public GameState AddTickOfResources (GameState state)
		{
			var activeConversions = new List<(string Conversion, double LeastAmount)> ();
			do {
				// Determine next tick
				var tickOfResources = CalculateAdditionalNextTick (state);
				var newResources = state.Resources.ToBuilder ();
				AddResources (newResources, tickOfResources);

				// If we're all positive, then go with that
				if (!newResources.Keys.Any(x => newResources[x] < 0))
					return state.WithResources (newResources.ToImmutable ());
			
				// Find the largets negative resource
				var leastResource = newResources.OrderBy (x => x.Value).First().Key;

				// Find the best conversion of that type that is enabled
				activeConversions.Clear ();
				foreach (var building in AllBuildings (state)) {
					foreach (var conversion in GetBuildingConversionResources (building).Where (x => IsConversionEnabled (state, x.Name))) {
						double amount = ResourceAmount (conversion.Resources, leastResource);
						activeConversions.Add ((conversion.Name, amount)); 
					}
				}
				string bestConversion = activeConversions.OrderBy (x => x.LeastAmount).First ().Conversion;

				// Disable that conversion
				state = state.WithDisabledConversions (state.DisabledConversions.Add (bestConversion));
			}
			while (activeConversions.Count > 0);
			
			return state;
		}

		public ImmutableDictionary<string, double> CalculateAdditionalNextTick (GameState state)
		{
			var additional = ImmutableDictionary.CreateBuilder<string, double> ();
			foreach (var building in AllBuildings (state) ) {
				AddResources (additional, GetBuildingResources (building));

				var conversions = GetBuildingConversionResources (building);
				foreach (var conversion in conversions.Where (x => IsConversionEnabled (state, x.Name)))
					AddResources (additional, conversion.Resources);
			}
			return additional.ToImmutable ();
		}

		public ImmutableDictionary<string, double> GetBuildingResources (string name)
		{
			var building = FindBuilding (name);
			return TotalYieldResources (building.Yield);
		}

		public List<(string Name, ImmutableDictionary<string, double> Resources)> GetBuildingConversionResources (string name)
		{
			var conversion = new List<(string name, ImmutableDictionary<string, double> resources)> ();
			var building = FindBuilding (name);
			foreach (var conversionYield in building.ConversionYield.AsNotNull ())
				conversion.Add ((conversionYield.Name, Yields.From (conversionYield)));
			return conversion;
		}
		
		public ImmutableDictionary<string, double> TotalYieldResources (Yield [] yields)
		{
			var resources = ImmutableDictionary.CreateBuilder<string, double> ();
			foreach (var yield in yields.AsNotNull ())
				AddResources (resources, Yields.From (yield));
			return resources.ToImmutable ();
		}

		public static double ResourceAmount (IDictionary<string, double> resources, string resourceName)
		{
			return resources.ContainsKey (resourceName) ? resources[resourceName] : 0;
		}
		
		public static void AddResources (ImmutableDictionary<string, double>.Builder left, IDictionary<string, double> right)
		{
			foreach (var resourceName in left.Keys.Union (right.Keys).ToList ())
			{
				double leftValue = ResourceAmount (left, resourceName);
				double rightValue = ResourceAmount (right, resourceName);
				left[resourceName] = leftValue + rightValue;
			}
		}
		
		public static void SubtractResources (ImmutableDictionary<string, double>.Builder left, IDictionary<string, double> right)
		{
			foreach (var resourceName in left.Keys.Union (right.Keys).ToList ())
			{
				double leftValue = ResourceAmount (left, resourceName);
				double rightValue = ResourceAmount (right, resourceName);
				left[resourceName] = leftValue - rightValue;
			}
		}
		
		public static bool HasMoreResources (ImmutableDictionary<string, double> left, IDictionary<string, double> right)
		{
			ImmutableDictionary<string, double>.Builder remain = left.ToBuilder ();
			SubtractResources (remain, right);
			foreach (var resourceName in right.Keys) {
				if (remain[resourceName] < 0)
					return false;
			}
			return true;		
		}

		IEnumerable<string> AllBuildings (GameState state)
		{
			foreach (var region in state.Regions)
				foreach (var area in region.Areas)
					foreach (var building in area.Buildings)
						yield return building;
		}

		public bool IsConversionEnabled (GameState state, string name) => !state.DisabledConversions.Contains (name);

		public List<(string Name, bool Enabled)> GetConversions (GameState state)
		{
			var consideredConversions = new HashSet<string> ();
			var allConversions = new List<(string Conversion, bool Enabled)> ();
			foreach (var building in AllBuildings (state)) {
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
