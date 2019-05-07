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

		ResourceEngine ResourceEngine;
		BuildingEngine BuildingEngine;
		public static GameEngine Create ()
		{
			var loader = JsonLoader.Load ();
			return new GameEngine (new ResourceEngine (loader));
		}

		public GameEngine (ResourceEngine resourceEngine)
		{
			ResourceEngine = resourceEngine;
			BuildingEngine = new BuildingEngine (ResourceEngine);
			RegionCapacity = ResourceEngine.RegionCapacity;
		}

		public GameState ApplyAction (GameState state, string action, string [] args = null)
		{
#if DEBUG
			Console.WriteLine ($"ApplyAction - {action}\nState = {JsonConvert.SerializeObject (state)}");
#endif
			switch (action)
			{
				case "Grow Population":
					Console.WriteLine ("Grow Population");
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
			return ResourceEngine.AddTickOfResources (state);
		}

		public List<(string BuildingName, ImmutableDictionary<string, double> Cost)> GetValidBuildingsForArea (Area area)
		{
			return BuildingEngine.GetValidBuildingsForArea (area);
		}

		public ImmutableDictionary<string, double> GetResourcesNextTick (GameState state)
		{
			return ResourceEngine.CalculateAdditionalNextTick (state);
		}
		
		public ImmutableDictionary<string, double> GetBuildingResources (string building)
		{
			return ResourceEngine.GetBuildingResources (building);
		}

		public static GameState CreateNewGame ()
		{
			var greenlandRegion = new Region ("Greenland", new Area[] { new Area (AreaType.Forest, new string[] { "Crude Workshop" }), new Area (AreaType.Plains), new Area (AreaType.Forest), new Area (AreaType.Forest), new Area (AreaType.Ocean) });
			var mudFlatsRegion = new Region ("Mudflats", new Area[] { new Area (AreaType.Swamp), new Area (AreaType.Swamp), new Area (AreaType.Swamp), new Area (AreaType.Plains), new Area (AreaType.Desert) });
			var resources = new Dictionary<string, double> { { "Food", 100 }, { "Wood", 50 } };
			return new GameState (Age.Stone, new Region[] { greenlandRegion, mudFlatsRegion }, resources);
		}
	}
}
