﻿using System;
using System.IO;
using System.Reflection;

using IncrementalSociety;
using IncrementalSociety.Model;
using IncrementalSociety.Json;

namespace IncrementalSociety.Runner
{
	class Program
	{
		static JsonLoader LoadXML ()
		{
			string assemblyPath = Path.GetDirectoryName (Assembly.GetExecutingAssembly().Location);
			string webRoot = Path.Combine (assemblyPath, "../../../../src/IncrementalSociety.Web/wwwroot/");
			string buildingsJson = File.ReadAllText (webRoot + "data/buildings.json");
			string gameJson = File.ReadAllText (webRoot + "data/game.json");
			string regionsJson = File.ReadAllText (webRoot + "data/regions.json");
			string resourcesJson = File.ReadAllText (webRoot + "data/resources.json");

			return new JsonLoader (buildingsJson, gameJson, regionsJson, resourcesJson);
		}

		static void Main(string[] args)
		{
			GameEngine engine = GameEngine.Create (LoadXML ());
			GameState state = engine.CreateNewGame ();
			var start = DateTime.Now;

			var buildOptions = new (string, int) [] {
				("Gathering Camp", 0),
				("Gathering Camp", 1),
				("Huts", 1),
				("Clay Pit", 2),
				("Gathering Camp", 2),
				("Gathering Camp", 3),
				("Clay Pit", 3)
			};
			int tickCount = 0;
			while (true) {
				if (tickCount % 1000 == 0 && tickCount < 7000) {
					int building = tickCount / 1000;
					var option = buildOptions[building];
					state = engine.ApplyAction (state, "Build District", new string [] { state.Regions[0].Name, option.Item2.ToString (), option.Item1 });
					while (engine.CanIncreasePopulationCap (state))
						state = engine.ApplyAction (state, "Grow Population Cap", new string [] {});
				}
				state = engine.ProcessTick (state);
				tickCount++;
				if ((DateTime.Now - start).Seconds > 10)
					break;
			}
			Console.WriteLine (tickCount);
		}
	}
}