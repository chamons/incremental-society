using System;
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
			GameState state = GameEngine.CreateNewGame ();
			GameEngine engine = GameEngine.Create (LoadXML ());
			var start = DateTime.Now;

			int tickCount = 0;
			while (true) {
				state = engine.ProcessTick (state);
				tickCount++;
				if ((DateTime.Now - start).Seconds > 10)
					break;
			}
			Console.WriteLine (tickCount);
		}
	}
}