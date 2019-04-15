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

		GameEngine Engine;
		public GameState State { get; private set; }

		public GameService ()
		{
			Loader = JsonLoader.Load ();
			State = GameEngine.CreateNewGame ();
			Engine = new GameEngine ();
		}

		// STUB_DATA - Filter by age
		public IEnumerable<ResourceDeclaration> Resources => Loader.Resources.Resources;
		public IEnumerable<GameAction> Actions => Loader.Actions.Actions;
		public IEnumerable<Region> Regions => State.Regions;
		public ImmutableDictionary<string, double> GetNextTickResources () => Engine.GetResourcesNextTick (State);

		public string GetImageFilename (ResourceDeclaration decl)
		{
			string name = decl.Name.ToLower ().Replace (' ', '-');
			if (decl.ImageHasAgePrefix)
				return $"images\\{State.Age}-{name}.png";
			else
				return $"images\\{name}.png";
		}

		public void ApplyAction (string action)
		{			
			State = Engine.ApplyAction (State, action);
		}

		public void OnTick ()
		{
			State = Engine.ProcessTick (State);
		}
	}
}
