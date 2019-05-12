using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using Newtonsoft.Json;

namespace IncrementalSociety
{
	public class GameEngine
	{
		public int RegionCapacity { get; private set; }

		PopulationEngine PopulationEngine;
		ResourceEngine ResourceEngine;
		BuildingEngine BuildingEngine;

		public static GameEngine Create (JsonLoader loader)
		{
			return new GameEngine (loader, new ResourceEngine (loader));
		}

		public GameEngine (JsonLoader loader, ResourceEngine resourceEngine)
		{
			ResourceEngine = resourceEngine;
			PopulationEngine = new PopulationEngine (ResourceEngine, loader);
			BuildingEngine = new BuildingEngine (ResourceEngine, PopulationEngine);
			RegionCapacity = ResourceEngine.RegionCapacity;
		}

		public GameState ApplyAction (GameState state, string action, string [] args = null)
		{
#if DEBUG
			Console.WriteLine ($"ApplyAction - {action}\nState = {JsonConvert.SerializeObject (state)}");
#endif
			switch (action)
			{
				case "Grow Population Cap":
					state = PopulationEngine.IncreasePopulationCap (state);
					break;
				case "Lower Population Cap":
					state = PopulationEngine.DecreasePopulationCap (state);
					break;
				case "Build District":
				{
					string regionName = args[0];
					int regionIndex = int.Parse (args[1]);
					string buildingName = args[2];
					state = BuildingEngine.Build (state, regionName, regionIndex, buildingName);
					break;
				}
				case "Destory District":
				{
					string regionName = args[0];
					int regionIndex = int.Parse (args[1]);
					int buildingIndex = int.Parse (args[2]);
					state = BuildingEngine.Destroy (state, regionName, regionIndex, buildingIndex);
					break;
				};
				default:
					throw new InvalidOperationException ($"Unable to find action {action}");
			}
#if DEBUG
			Console.WriteLine ($"ApplyAction Done\nState = {JsonConvert.SerializeObject (state)}");
#endif
			return state;
		}
		
		public bool IsConversionEnabled (GameState state, string name) => ResourceEngine.IsConversionEnabled (state, name);
		
		public GameState ToggleConversion (GameState state, string conversion)
		{
			if (IsConversionEnabled (state, conversion))
				return state.WithDisabledConversions (state.DisabledConversions.Add (conversion));
			else
				return state.WithDisabledConversions (state.DisabledConversions.Remove (conversion));
		}
		
		public List<(string Name, bool Enabled)> GetConversions (GameState state) => ResourceEngine.GetConversions (state);
		
		public List<(string Name, ImmutableDictionary<string, double> Resources)> GetBuildingConversionResources (string name)
		{
			return ResourceEngine.GetBuildingConversionResources (name);
		}

		public GameState ProcessTick (GameState state)
		{
			double efficiency = PopulationEngine.GetPopulationEfficiency (state);
			return ResourceEngine.AddTickOfResources (state, efficiency);
		}

		public List<(string BuildingName, ImmutableDictionary<string, double> Cost)> GetValidBuildingsForArea (Area area)
		{
			return BuildingEngine.GetValidBuildingsForArea (area);
		}
		
		public bool CanAffordBuilding (GameState state, string buildingName) => BuildingEngine.CanAffordBuilding (state, buildingName);

		public ImmutableDictionary<string, double> GetResourcesNextTick (GameState state)
		{
			double efficiency = PopulationEngine.GetPopulationEfficiency (state);
			return ResourceEngine.CalculateAdditionalNextTick (state, efficiency);
		}
		
		public ImmutableDictionary<string, double> GetBuildingResources (string building)
		{
			return ResourceEngine.GetBuildingResources (building);
		}

		public const int CurrentVersion = 1; 

		public static GameState CreateNewGame ()
		{
			var greenlandRegion = new Region ("Greenland", new Area[] { new Area (AreaType.Forest, new string[] { "Crude Settlement" }), new Area (AreaType.Plains), new Area (AreaType.Forest), new Area (AreaType.Forest), new Area (AreaType.Ocean) });
			var mudFlatsRegion = new Region ("Mudflats", new Area[] { new Area (AreaType.Swamp), new Area (AreaType.Swamp), new Area (AreaType.Swamp), new Area (AreaType.Plains), new Area (AreaType.Desert) });
			var resources = new Dictionary<string, double> { { "Food", 100 }, { "Wood", 50 } };
			return new GameState (CurrentVersion, Age.Stone, new Region[] { greenlandRegion, mudFlatsRegion }, resources, 1000, 2000);
		}
	}
}
