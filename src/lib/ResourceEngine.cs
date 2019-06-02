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
		Dictionary<string, Building> BuildingLookup;
		Dictionary<string, Resources> AreaBonuses;
		Dictionary<string, Resources> FeatureBonuses;
		JsonLoader Json;

		public ResourceConfig ResourceConfig;

		public IEnumerable <Building> Buildings => BuildingLookup.Values;

		public ResourceEngine (JsonLoader json)
		{
			Json = json;
			BuildingLookup = json.Buildings.Buildings.ToDictionary (x => x.Name, x => x);
			ResourceConfig = new ResourceConfig (json.Resources.Resources.Select (x => x.Name));
			AreaBonuses = json.Areas.Areas.AsNotNull ().ToDictionary (x => x.Name, x => GetBaseResources (x.BonusYield));
			FeatureBonuses = json.Areas.Features.AsNotNull ().ToDictionary(x => x.Name, x => GetBaseResources(x.BonusYield));
		}

		public Building FindBuilding (string name)
		{
			var building = BuildingLookup [name];
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
					foreach (var conversion in GetBuildingConversionResources (state, building).Where (x => IsConversionEnabled (state, x.Name)))
						if (conversion.Resources[missingResource] < 0)
							activeConversions.Add ((conversion.Name, conversion.Resources[missingResource]));
				return activeConversions;
		}

		public Resources GetAreaBonus(Area area)
		{
			var bonus = AreaBonuses[area.Type].ToBuilder();
			foreach (var feature in area.Features.AsNotNull()) {
				bonus.MultiplyEmptyAsOne (FeatureBonuses[feature]);
			}
				
			return bonus.ToResources();
		}

		public Resources CalculateAdditionalNextTick (GameState state, double efficiency)
		{
			var additional = ResourceConfig.CreateBuilder ();
			foreach (var region in state.Regions) {
				foreach (var area in region.Areas) {
					var areaBonus = AreaBonuses[area.Type];
					foreach (var building in area.Buildings) {
						var buildingResources = GetBuildingResources(state, building).ToBuilder();
						buildingResources.Multiply (areaBonus);

						foreach (var feature in area.Features.AsNotNull())
							buildingResources.Multiply(FeatureBonuses[feature]);

						additional.AddWithMultiply (buildingResources, efficiency);

						var conversions = GetBuildingConversionResources (state, building);
						foreach (var conversion in conversions.Where (x => IsConversionEnabled (state, x.Name)))
							additional.Add (conversion.Resources);
					}
				}
			}

			return additional.ToResources ();
		}

		public Resources GetBaseResources (IEnumerable<Yield> allYields)
		{
			return ResourceConfig.Create (allYields);
		}

		public Resources GetResourcesBasedOnTech (GameState state, IEnumerable<Yield> allYields)
		{
			var yields = allYields.AsNotNull ().Where (x => state.HasResearch (x.RequireTechnology));
			return ResourceConfig.Create (yields);
		}

		public int GetCapacityBasedOnTech (GameState state, IEnumerable<CapacityDeclaration> capacities)
		{
			return capacities.AsNotNull ().Where (x => state.HasResearch (x.RequireTechnology)).Sum (x => x.Capacity);
		}

		public Resources GetBuildingResources (GameState state, string name) => GetResourcesBasedOnTech (state, FindBuilding (name).Yield);
		public Resources GetBuildingResources (GameState state, Building building) => GetResourcesBasedOnTech (state, building.Yield);

		public Resources GetBuildingStorage (GameState state, string name) => GetResourcesBasedOnTech (state, FindBuilding (name).Storage);
		public Resources GetBuildingStorage (GameState state, Building building) => GetResourcesBasedOnTech (state, building.Storage);

		public Resources GetBuildingCost (GameState state, string name) => GetResourcesBasedOnTech (state, FindBuilding (name).Cost);
		public Resources GetBuildingCost (GameState state, Building building) => GetResourcesBasedOnTech (state, building.Cost);

		public double GetBuildingHousing (GameState state, string name) => GetCapacityBasedOnTech (state, FindBuilding (name).HousingCapacity);
		public double GetBuildingHousing (GameState state, Building building) => GetCapacityBasedOnTech (state, building.HousingCapacity);

		public int GetRegionCapacity (GameState state) => GetCapacityBasedOnTech (state, Json.Game.RegionCapacityDeclarations);

		public List<(string Name, Resources Resources)> GetBuildingConversionResources (GameState state, string name)
		{
			var conversion = new List<(string name, Resources resources)> ();
			var building = FindBuilding (name);
			foreach (var conversionYield in building.ConversionYield.AsNotNull ()) {
				var costs = GetResourcesBasedOnTech (state, conversionYield.Cost);
				var provided = GetResourcesBasedOnTech (state, conversionYield.Provides);
				conversion.Add ((conversionYield.Name, provided.Subtract (costs)));
			}
			return conversion;
		}

		public bool IsConversionEnabled (GameState state, string name) => !state.DisabledConversions.Contains (name);

		public List<(string Name, bool Enabled)> GetConversions (GameState state)
		{
			var consideredConversions = new HashSet<string> ();
			var allConversions = new List<(string Conversion, bool Enabled)> ();
			foreach (var building in state.AllBuildings()) {
				foreach (var conversion in GetBuildingConversionResources (state, building)) {
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

			foreach (var building in state.AllBuildings ())
				storage.Add (GetBuildingStorage (state, building));
			return storage.ToResources ();
		}
	}
}
