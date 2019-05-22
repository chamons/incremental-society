using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Net.Http;
using System.Threading.Tasks;

using Microsoft.AspNetCore.Components;

using IncrementalSociety.Json;
using IncrementalSociety.Model;
using Microsoft.JSInterop;
using Newtonsoft.Json;

namespace IncrementalSociety.Web.Services
{
	public enum GameUIState {
		Default,
		SelectRegionToBuildIn,
		ShowBuildingSelectDialog,
		SelectBuildingToDestory,
		InternalError
	}

	public class GameUIStateChangedEventArgs : EventArgs
	{
		public Dictionary<string, object> Options { get; set; }
	}

	public class GameService
	{
		HttpClient Client;
		IUriHelper URIHelper;
		IJSRuntime JSRuntime;

		public JsonLoader Loader { get; private set; }
		public GameEngine Engine { get; private set; }

		public event EventHandler<GameUIStateChangedEventArgs> CurrentUIStateChanged;
		public GameUIState CurrentUIState { get; private set; } = GameUIState.Default;

		public GameState State { get; private set; }
		public bool Loaded { get; private set; }

		public GameService (HttpClient client, IUriHelper uriHelper, IJSRuntime jsRuntime, ExceptionNotificationService exceptionNotification)
		{
			Client = client;
			URIHelper = uriHelper;
			JSRuntime = jsRuntime;
			exceptionNotification.OnException += (o, s) => OnException (s);
		}

		void OnException (string s)
		{
			var options = new Dictionary<string, object> {
				["Exception"] = s
			};
			SetUIState (GameUIState.InternalError, options);
		}

		public async Task Load ()
		{
			Loader = await LoadXML ();
			Engine = GameEngine.Create (Loader);

			try {
				string serializedState = ((IJSInProcessRuntime)JSRuntime).Invoke<string> ("LoadGame");
				if (!string.IsNullOrEmpty (serializedState) && serializedState != "null") {
					State = JsonConvert.DeserializeObject<GameState> (serializedState);
					if (State.Version != GameEngine.CurrentVersion)
						State = null;
				}
			}
			catch (Exception e) {
				Console.Error.WriteLine ($"Error loading game: {e.Message}");
			}

			if (State == null)
				State = Engine.CreateNewGame ();
			Loaded = true;
		}

		async Task<JsonLoader> LoadXML ()
		{
			string buildingsJson = await Client.GetStringAsync (URIHelper.GetBaseUri () + "data/buildings.json");
			string gameJson = await Client.GetStringAsync (URIHelper.GetBaseUri () + "data/game.json");
			string regionsJson = await Client.GetStringAsync (URIHelper.GetBaseUri () + "data/regions.json");
			string resourcesJson = await Client.GetStringAsync (URIHelper.GetBaseUri () + "data/resources.json");
			string researchJson = await Client.GetStringAsync (URIHelper.GetBaseUri () + "data/research.json");

			return new JsonLoader (buildingsJson, gameJson, regionsJson, resourcesJson, researchJson);
		}

		// Shared between multiple consumers
		public List<(string Name, bool Enabled)> Conversions => Engine.GetConversions (State);
		public Resources GetNextTickResources () => Engine.GetResourcesNextTick (State);
		public Resources GetResourceStorage () => Engine.GetResourceStorage (State);

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
			CurrentUIState = state;
			Refresh (options);
		}

		void Refresh (Dictionary<string, object> options = null)
		{
			CurrentUIStateChanged?.Invoke (this, new GameUIStateChangedEventArgs () { Options = options });
		}

		public void NewGame ()
		{
			State = Engine.CreateNewGame ();
			SetUIState (GameUIState.Default);
		}

		public void Save ()
		{
			((IJSInProcessRuntime)JSRuntime).Invoke<object> ("SaveGame", JsonConvert.SerializeObject (State) );
		}
	}
}
