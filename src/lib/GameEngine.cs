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
		ResourceEngine ResourceEngine;

		public static GameEngine Create ()
		{
			var loader = JsonLoader.Load ();
			return new GameEngine (new ResourceEngine (loader));
		}

		public GameEngine (ResourceEngine resourceEngine)
		{
			ResourceEngine = resourceEngine;
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
					Console.WriteLine ("Build");
					break;
				case "Destory District":
					Console.WriteLine ("Nuke");
					break;
				default:
					throw new InvalidOperationException ($"Unable to find action {action}");
			}
#if DEBUG
			Console.WriteLine ($"ApplyAction Done\nState = {JsonConvert.SerializeObject (state)}");
#endif
			return state;
		}

		public GameState ProcessTick (GameState state)
		{
			return ResourceEngine.AddTickOfResources (state);
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
