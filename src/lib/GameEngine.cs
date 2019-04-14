using System;
using System.Collections.Generic;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using Newtonsoft.Json;

namespace IncrementalSociety
{
	public class GameEngine
	{
		public static GameState ApplyAction (GameState state, string action)
		{
#if DEBUG
			Console.WriteLine ($"ApplyAction - {action}\nState = {JsonConvert.SerializeObject (state)}");
#endif
			switch (action)
			{
				case "Pass Time":
					state = ProcessTick (state);
					break;
				case "Grow Population":
					Console.WriteLine ("Grow Population");
					break;
				default:
					throw new InvalidOperationException ($"Unable to find action {action}");
			}
#if DEBUG
			Console.WriteLine ($"ApplyAction Done\nState = {JsonConvert.SerializeObject (state)}");
#endif
			return state;
		}

		public static GameState ProcessTick (GameState state)
		{
			JsonLoader loader = JsonLoader.Load ();
			ResourceEngine engine = new ResourceEngine (loader);
			return engine.AddTickOfResources (state);
		}

		public static GameState CreateNewGame ()
		{
			var greenlandRegion = new Region ("Greenland", new Area[] { new Area (AreaType.Forest, new string[] { "Crude Settlement" }), new Area (AreaType.Plains), new Area (AreaType.Forest), new Area (AreaType.Forest), new Area (AreaType.Ocean) });
			var mudFlatsRegion = new Region ("Mudflats", new Area[] { new Area (AreaType.Swamp), new Area (AreaType.Swamp), new Area (AreaType.Swamp), new Area (AreaType.Plains), new Area (AreaType.Desert) });
			var resources = new Dictionary<string, double> { { "Food", 100 }, { "Wood", 50 } };
			return new GameState (Age.Stone, new Region[] { greenlandRegion, mudFlatsRegion }, resources);
		}
	}
}
