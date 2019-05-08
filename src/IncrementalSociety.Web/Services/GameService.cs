using System;
using System.Collections.Generic;
using System.Collections.Immutable;

using IncrementalSociety.Json;
using IncrementalSociety.Model;

namespace IncrementalSociety.Web.Services
{
	public enum GameUIState { Default, SelectRegionToBuildIn, ShowBuildingSelectDialog, SelectBuildingToDestory }
	
	public class GameUIStateChangedEventArgs : EventArgs
	{
		public Dictionary<string, object> Options { get; set; }
	}
	
	public class GameService
	{
		public JsonLoader Loader { get; private set; }
		public GameEngine Engine { get; private set; } 

		public event EventHandler<GameUIStateChangedEventArgs> CurrentUIStateChanged;  
		public GameUIState CurrentUIState { get; private set; } = GameUIState.Default;

		public GameState State { get; private set; }
	
		public GameService ()
		{
			Loader = JsonLoader.Load ();
			State = GameEngine.CreateNewGame ();
			Engine = GameEngine.Create ();
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
