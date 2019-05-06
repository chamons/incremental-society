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

			Building building = ResourceEngine.FindBuilding (buildingName);
			if (building == null)
				throw new InvalidOperationException ($"Build for {buildingName} but unable to find.");

			if (!building.ValidRegions.Contains (area.Type.ToString()))
				throw new InvalidOperationException ($"Build for {buildingName} but wrong region {area.Type}.");
			
			// Can we simplify this?
			var newArea = area.WithBuildings (area.Buildings.Add (building.Name));
			var newAreas = region.Areas.Replace (area, newArea);
			var newRegion = region.WithAreas (newAreas);
			var newRegions = state.Regions.Replace (region, newRegion);
			return state.WithRegions (newRegions);
		}

		public GameState Destroy (GameState state, string regionName, int regionIndex, int buildingIndex)
		{
			return state;
		}
	}
}
