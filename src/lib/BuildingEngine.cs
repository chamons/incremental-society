using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

using IncrementalSociety.Population;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

namespace IncrementalSociety
{
	public class BuildingEngine
	{
		PopulationEngine PopulationEngine;
		ResourceEngine ResourceEngine;

		ResourceConfig ResourceConfig => ResourceEngine.ResourceConfig;

		public BuildingEngine (ResourceEngine engine, PopulationEngine populationEngine)
		{
			ResourceEngine = engine;
			PopulationEngine = populationEngine;
		}

		public GameState Build (GameState state, string regionName, int regionIndex, string buildingName)
		{
			Region region = state.Regions.First (x => x.Name == regionName);
			Area area = region.Areas [regionIndex];
			if (area.Buildings.Length >= ResourceEngine.GetRegionCapacity (state))
				throw new InvalidOperationException ($"Build in {regionName} {regionIndex} for {buildingName} but out of room {area.Buildings.Length}");

			var building = ResourceEngine.FindBuilding (buildingName);

			if (!CanBuildBuilding (state, building, area))
				throw new InvalidOperationException ($"Build in {regionName} {regionIndex} but {buildingName} is unable to build?");

			var buildingTotalCost = ResourceEngine.GetBuildingCost (state, building);
			if (!state.Resources.HasMoreThan (buildingTotalCost))
				throw new InvalidOperationException ($"Build for {buildingName} but not enough resourcs.");
			var newResouces = state.Resources.ToBuilder ();
			newResouces.Subtract (buildingTotalCost);
			state = state.WithResources (newResouces.ToResources ());

			var newArea = area.WithBuildings (area.Buildings.Add (building.Name));
			return UpdateStateWithArea (state, area, newArea, region);
		}

		public bool CanAffordBuilding (GameState state, string buildingName)
		{
			var buildingTotalCost = ResourceEngine.GetBuildingCost (state, buildingName);
			return state.Resources.HasMoreThan (buildingTotalCost);
		}

		public GameState Destroy (GameState state, string regionName, int regionIndex, int buildingIndex)
		{
			Region region = state.Regions.First (x => x.Name == regionName);
			Area area = region.Areas [regionIndex];
			if (buildingIndex >= area.Buildings.Length)
				throw new InvalidOperationException ($"Destroy in {regionName} {regionIndex} for but invalid index {buildingIndex}");

			string buildingName = area.Buildings[buildingIndex];

			var building = ResourceEngine.FindBuilding (buildingName);
			if (building.PreventDestroy)
				throw new InvalidOperationException ($"Destroy in {regionName} {regionIndex} but {buildingName} is marked unable to destory");


			var newArea = area.WithBuildings (area.Buildings.Remove (buildingName));
			return UpdateStateWithArea (state, area, newArea, region);
		}

		GameState UpdateStateWithArea (GameState state, Area area, Area newArea, Region region)
		{
			var newAreas = region.Areas.Replace (area, newArea);
			var newRegion = region.WithAreas (newAreas);
			var newRegions = state.Regions.Replace (region, newRegion);
			return state.WithRegions (newRegions);
		}

		bool BuildingValidForArea (Building building, Area area)
		{
			return building.ValidAreas.Contains ("Any") || building.ValidAreas.Contains (area.Type.ToString ());
		}

		bool CanBuildBuilding (GameState state, Building building, Area area)
		{
			if (building.PreventBuild)
				return false;
			if (!BuildingValidForArea (building, area))
				return false;
			if (!state.HasResearch (building.RequireTechnology))
				return false;
			return true;
		}

		public List<string> GetValidBuildingsForArea (GameState state, Area area)
		{
			IEnumerable<Building> buildings = ResourceEngine.Buildings.Where (x => CanBuildBuilding (state, x, area));
			return buildings.Select (b => b.Name).ToList ();
		}
	}
}
