using System.Collections.Generic;
using System.Linq;

using IncrementalSociety.Model;
using IncrementalSociety.Json;
using System.Collections.Immutable;

namespace IncrementalSociety.Web.Services
{
	public class GameService
	{
		JsonLoader Loader;

		public GameState State { get; private set; }

		public GameService ()
		{
			Loader = JsonLoader.Load ();
			State = GameEngine.CreateNewGame ();
		}

		// STUB_DATA - Filter by age
		public IEnumerable<ResourceDeclaration> Resources => Loader.Resources.Resources;
		public IEnumerable<GameAction> Actions => Loader.Actions.Actions;
		public IEnumerable<Region> Regions => State.Regions;
		public ImmutableDictionary<string, double> GetNextTickResources () => GameEngine.GetResourcesNextTick (State);

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
