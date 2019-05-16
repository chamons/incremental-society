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
		PopulationEngine PopulationEngine;
		ResourceEngine ResourceEngine;
		YieldCache Yields;

		public BuildingEngine (ResourceEngine engine, PopulationEngine populationEngine)
		{
			ResourceEngine = engine;
			PopulationEngine = populationEngine;
			Yields = new YieldCache ();
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
			
			var buildingTotalCost = Yields.Total (building.Cost);
			if (!state.Resources.HasMoreThan (buildingTotalCost))
				throw new InvalidOperationException ($"Build for {buildingName} but not enough resourcs.");
			var newResouces = state.Resources.ToBuilder ();
			newResouces.Subtract (buildingTotalCost);
			state = state.WithResources (newResouces.ToImmutable ());

			var newArea = area.WithBuildings (area.Buildings.Add (building.Name));
			return UpdateStateWithArea (state, area, newArea, region);
		}

		public bool CanAffordBuilding (GameState state, string buildingName)
		{
			var building = ResourceEngine.FindBuilding (buildingName);
			var buildingTotalCost = Yields.Total (building.Cost);
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
		
		public List<(string BuildingName, ImmutableDictionary<string, double> Cost)> GetValidBuildingsForArea (Area area)
		{
			IEnumerable<Building> buildings = ResourceEngine.Buildings.Where (x => x.ValidRegions.Contains (area.Type.ToString ()));
			return buildings.Select (b => (b.Name, b.Cost?.ToImmutableDictionary (x => x.Name, x => x.Amount))).ToList ();
		}
	}
}
