using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

namespace IncrementalSociety
{
	public class BuildingEngine
	{
		ResourceEngine ResourceEngine;
		public BuildingEngine (ResourceEngine engine)
		{
			ResourceEngine = engine;
		}

		public GameState Build (GameState state, string regionName, int regionIndex, string buildingName)
		{
			Region region = state.Regions.First (x => x.Name == regionName); 
			Area area = region.Areas [regionIndex];
			if (area.Buildings.Length >= ResourceEngine.RegionCapacity)
				throw new InvalidOperationException ($"Build in {regionName} {regionIndex} for {buildingName} but out of room {area.Buildings.Length}");

			var building = ResourceEngine.FindBuilding (buildingName);

			if (!building.ValidRegions.Contains (area.Type.ToString()))
				throw new InvalidOperationException ($"Build for {buildingName} but wrong region {area.Type}.");
			
			var buildingTotalCost = ResourceEngine.TotalYieldResources (building.Cost);
			if (!ResourceEngine.HasMoreResources (state.Resources, buildingTotalCost))
				throw new InvalidOperationException ($"Build for {buildingName} but not enough resourcs.");
			var newResouces = state.Resources.ToBuilder ();
			ResourceEngine.SubtractResources (newResouces, buildingTotalCost);
			state = state.WithResources (newResouces.ToImmutable ());

			var newArea = area.WithBuildings (area.Buildings.Add (building.Name));
			return UpdateStateWithArea (state, area, newArea, region);
		}

		public bool CanAffordBuilding (GameState state, string buildingName)
		{
			var building = ResourceEngine.FindBuilding (buildingName);
			var buildingTotalCost = ResourceEngine.TotalYieldResources (building.Cost);
			return ResourceEngine.HasMoreResources (state.Resources, buildingTotalCost);
		}

		public GameState Destroy (GameState state, string regionName, int regionIndex, int buildingIndex)
		{
			Region region = state.Regions.First (x => x.Name == regionName); 
			Area area = region.Areas [regionIndex];
			if (buildingIndex >= area.Buildings.Length)
				throw new InvalidOperationException ($"Destroy in {regionName} {regionIndex} for but invalid index {buildingIndex}");

			string buildingName = area.Buildings[buildingIndex];

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
		
		public List<(string BuildingName, ImmutableDictionary<string, double> Cost)> GetValidBuildingsForArea (Area area)
		{
			IEnumerable<Building> buildings = ResourceEngine.Buildings.Where (x => x.ValidRegions.Contains (area.Type.ToString ()));
			return buildings.Select (b => (b.Name, b.Cost?.ToImmutableDictionary (x => x.Name, x => x.Amount))).ToList ();
		}
	}
}
