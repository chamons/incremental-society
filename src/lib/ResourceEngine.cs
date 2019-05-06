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
			return Json.Buildings.Buildings.FirstOrDefault (x => x.Name == name);
		}
		
		public Settlement FindSettlement (string name)
		{
			return Json.Buildings.Settlements.FirstOrDefault (x => x.Name == name);
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
			foreach (var region in state.Regions)
			{
				foreach (var area in region.Areas)
				{
					foreach (var building in area.Buildings)
					{
						AddResources (additional, GetBuildingResources (building));
					}
				}
			}
			return additional.ToImmutable ();
		}

		// TODO Pass in activated conversions
		public ImmutableDictionary<string, double> GetBuildingResources (string name)
		{
			var resources = ImmutableDictionary.CreateBuilder<string, double> ();
			var building = FindBuilding (name);
			if (building != null)
			{
				foreach (var yield in building.Yield.AsNotNull ())
					AddResources (resources, Yields.From (yield));

				foreach (var conversionYield in building.ConversionYield.AsNotNull ())
					AddResources (resources, Yields.From (conversionYield));

				return resources.ToImmutable ();
			}

			var settlement = FindSettlement (name);
			if (settlement != null)
			{
				foreach (var yield in settlement.Yield.AsNotNull ())
					AddResources (resources, Yields.From (yield));
				return resources.ToImmutable ();
			}
			throw new InvalidOperationException ($"Unable to find building \"{name}\" in resources");
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
