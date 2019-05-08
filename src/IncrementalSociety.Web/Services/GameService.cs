using System;
using System.Collections.Generic;
using System.Linq;

using IncrementalSociety.Model;
using IncrementalSociety.Json;
using System.Collections.Immutable;

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
		public int RegionCapacity => Engine.RegionCapacity;
	
		public GameService ()
		{
			Loader = JsonLoader.Load ();
			State = GameEngine.CreateNewGame ();
			Engine = GameEngine.Create ();
		}

		// STUB_DATA - Filter by age
		public IEnumerable<ResourceDeclaration> Resources => Loader.Resources.Resources;
		public IEnumerable<Region> Regions => State.Regions;
		public ImmutableDictionary<string, double> GetNextTickResources () => Engine.GetResourcesNextTick (State);
		
		public ImmutableDictionary<string, double> GetBuildingResources (string building) => Engine.GetBuildingResources (building);
		public List<(string Name, ImmutableDictionary<string, double> Resources)> GetBuildingConversionResources (string name)
		{
			return Engine.GetBuildingConversionResources (name);
		}
		
		public List<(string Name, bool Enabled)> Conversions => Engine.GetConversions (State);
		
		public bool IsConversionEnabled (string name) => Engine.IsConversionEnabled (State, name);
		
		public void ToggleConversion (string conversion)
		{
			State = Engine.ToggleConversion (State, conversion);
			SetUIState (GameUIState.Default);
		}

		public void ApplyAction (string action, string[] args = null)
		{
			State = Engine.ApplyAction (State, action, args);
			Refresh ();
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
	
		public void OnTick ()
		{
			State = Engine.ProcessTick (State);
			Refresh ();
		}
	}
}
