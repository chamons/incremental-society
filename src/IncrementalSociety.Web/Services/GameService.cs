using System.Collections.Generic;
using System.Linq;

using IncrementalSociety.Model;
using IncrementalSociety.Json;

namespace IncrementalSociety.Web.Services
{
	public class GameService
	{
		JsonLoader Loader;
		GameState State;

		public GameService ()
		{
			Loader = JsonLoader.Load ();
			State = GameEngine.CreateNewGame ();
		}

		// STUB_DATA - Filter by age
		public IEnumerable<ResourceDeclaration> Resources => Loader.Resources.Resources;

		public IEnumerable<GameAction> Actions => Loader.Actions.Actions;

		public IEnumerable<Region> Regions => State.Regions;

		public double GetResourceCount (string name)
		{
			return State.Resources.ContainsKey (name) ? State.Resources[name] : 0;
		}

		public double GetNextTickResourceCount (string name)
		{
			var resources = GameEngine.GetResourcesNextTick (State);
			return resources.ContainsKey (name) ? resources[name] : 0;
		}

		public string GetImageFilename (ResourceDeclaration decl)
		{
			string name = decl.Name.ToLower ().Replace (' ', '-');
			if (decl.Image_has_age_prefix)
				return $"images\\{State.Age}-{name}.png";
			else
				return $"images\\{name}.png";
		}

		public void ApplyAction (string action)
		{			
			State = GameEngine.ApplyAction (State, action);
		}
	}
}
