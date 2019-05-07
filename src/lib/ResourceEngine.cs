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
			var newResources = state.Resources.ToBuilder ();
			AddResources (newResources, CalculateAdditionalNextTick (state));
			return state.WithResources (newResources.ToImmutable ());
		}

		public ImmutableDictionary<string, double> CalculateAdditionalNextTick (GameState state)
		{
			var additional = ImmutableDictionary.CreateBuilder<string, double> ();
			foreach (var region in state.Regions) {
				foreach (var area in region.Areas) {
					foreach (var building in area.Buildings) {
						AddResources (additional, GetBuildingResources (building));
						var conversions = GetBuildingConvertedResources (building);
						foreach (var conversion in conversions) {
							if (!state.DisabledConversions.Contains (conversion.Name)) {
								AddResources (additional, conversion.Resources);
							}
						}
					}
				}
			}
			return additional.ToImmutable ();
		}

		public ImmutableDictionary<string, double> GetBuildingResources (string name)
		{
			var building = FindBuilding (name);
			return TotalYieldResources (building.Yield);
		}

		public List<(string Name, ImmutableDictionary<string, double> Resources)> GetBuildingConvertedResources (string name)
		{
			var conversion = new List<(string name, ImmutableDictionary<string, double> resources)> ();
			var building = FindBuilding (name);
			foreach (var conversionYield in building.ConversionYield.AsNotNull ())
				conversion.Add ((conversionYield.Name, Yields.From (conversionYield)));
			return conversion;
		}
		
		ImmutableDictionary<string, double> TotalYieldResources (Yield [] yields)
		{
			var resources = ImmutableDictionary.CreateBuilder<string, double> ();
			foreach (var yield in yields.AsNotNull ())
				AddResources (resources, Yields.From (yield));
			return resources.ToImmutable ();
		}
		
		public static void AddResources (ImmutableDictionary<string, double>.Builder left, IDictionary<string, double> right)
		{
			foreach (var resourceName in left.Keys.Union (right.Keys).ToList ())
			{
				double leftValue = left.ContainsKey (resourceName) ? left[resourceName] : 0;
				double rightValue = right.ContainsKey (resourceName) ? right[resourceName] : 0;
				left[resourceName] = leftValue + rightValue;
			}
		}
	}
}
