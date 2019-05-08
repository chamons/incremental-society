using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Net.Http;
using System.Threading.Tasks;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using Microsoft.AspNetCore.Components;

namespace IncrementalSociety.Web.Services
{
	public enum GameUIState { Default, SelectRegionToBuildIn, ShowBuildingSelectDialog, SelectBuildingToDestory }
	
	public class GameUIStateChangedEventArgs : EventArgs
	{
		public Dictionary<string, object> Options { get; set; }
	}
	
	public class GameService
	{
		HttpClient Client;
		IUriHelper URIHelper;
		public JsonLoader Loader { get; private set; }
		public GameEngine Engine { get; private set; } 

		public event EventHandler<GameUIStateChangedEventArgs> CurrentUIStateChanged;  
		public GameUIState CurrentUIState { get; private set; } = GameUIState.Default;

		public GameState State { get; private set; }
		public bool Loaded { get; private set; }
	
		public GameService (HttpClient client, IUriHelper uriHelper)
		{
			Client = client;
			URIHelper = uriHelper;
		}

		public async Task Load ()
		{
			string actionsJson = await Client.GetStringAsync (URIHelper.GetBaseUri () + "data/actions.json");
			Console.Error.WriteLine (URIHelper.GetBaseUri () + "data/actions.json");
			string buildingsJson = await Client.GetStringAsync (URIHelper.GetBaseUri () + "data/buildings.json");
			string gameJson = await Client.GetStringAsync (URIHelper.GetBaseUri () + "data/game.json");
			string regionsJson = await Client.GetStringAsync (URIHelper.GetBaseUri () + "data/regions.json");
			string resourcesJson = await Client.GetStringAsync (URIHelper.GetBaseUri () + "data/resources.json");

			Loader = new JsonLoader (actionsJson, buildingsJson, gameJson, regionsJson, resourcesJson);
			State = GameEngine.CreateNewGame ();
			Engine = GameEngine.Create (Loader);
			Loaded = true;
		}

		// Shared between multiple consumers
		public List<(string Name, bool Enabled)> Conversions => Engine.GetConversions (State);
		public ImmutableDictionary<string, double> GetNextTickResources () => Engine.GetResourcesNextTick (State);

		public void OnTick ()
		{
			State = Engine.ProcessTick (State);
			Refresh ();
		}

		public void ApplyAction (string action, string[] args = null)
		{
			State = Engine.ApplyAction (State, action, args);
			Refresh ();
		}

		public void ToggleConversion (string conversion)
		{
			State = Engine.ToggleConversion (State, conversion);
			SetUIState (GameUIState.Default);
		}

		public void SetUIState (GameUIState state, Dictionary<string, object> options = null)
		{
#if DEBUG
			Console.Error.WriteLine ($"SetUIState: {state}");
#endif
			CurrentUIState = state;

			Refresh (options);
		}

		void Refresh (Dictionary<string, object> options = null)
		{
			CurrentUIStateChanged?.Invoke (this, new GameUIStateChangedEventArgs () { Options = options });
		}
	}
}
