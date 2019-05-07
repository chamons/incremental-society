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
		public Action NotifyUIStateHasChanged;

		public event EventHandler<GameUIStateChangedEventArgs> CurrentUIStateChanged;  
		public GameUIState CurrentUIState { get; private set; } = GameUIState.Default;
		public GameState State { get; private set; }
		public int RegionCapacity => Engine.RegionCapacity;
	
		const string CancelText = "Cancel";
		// These must match keys in GameEngine::ApplyAction
		const string BuildText = "Build District";
		const string DestroyText = "Destory District";

		public GameService ()
		{
			Loader = JsonLoader.Load ();
			State = GameEngine.CreateNewGame ();
			Engine = GameEngine.Create ();
			
			ResetActionList ();
		}

		void ResetActionList ()
		{
			Actions = new List<string> (Loader.Actions.Actions.Select (x => x.Name));
			Actions.Add (BuildText);
			Actions.Add (DestroyText);
		}

		// STUB_DATA - Filter by age
		public IEnumerable<ResourceDeclaration> Resources => Loader.Resources.Resources;
		public IEnumerable<Region> Regions => State.Regions;
		public ImmutableDictionary<string, double> GetNextTickResources () => Engine.GetResourcesNextTick (State);
		
		public List<string> Actions { get; private set; }

		public ImmutableDictionary<string, double> GetBuildingResources (string building) => Engine.GetBuildingResources (building);
		public List<(string Name, ImmutableDictionary<string, double> Resources)> GetBuildingConversionResources (string name)
		{
			return Engine.GetBuildingConversionResources (name);
		}

		public void ApplyAction (string action)
		{
			switch (action)
			{
				case BuildText:
					SetUIState (GameUIState.SelectRegionToBuildIn);
					return;
				case DestroyText:
					SetUIState (GameUIState.SelectBuildingToDestory);
					return;
				case CancelText:
					SetUIState (GameUIState.Default);
					return;
				default:
					SetUIState (GameUIState.Default);
					State = Engine.ApplyAction (State, action);
					return;
			}
		}
		
		public List<(string Name, bool Enabled)> Conversions => Engine.GetConversions (State);
		
		public bool IsConversionEnabled (string name) => Engine.IsConversionEnabled (State, name);
		
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
			
			ResetActionList ();
			ReplaceActionWithCancel (state);
			NotifyUIStateHasChanged ();
			CurrentUIStateChanged?.Invoke (this, new GameUIStateChangedEventArgs () { Options = options });
		}

		void ReplaceActionWithCancel (GameUIState state)
		{
			string actionText = GetActionTextForState (state);
			if (actionText != null) {
				for (int i = 0 ; i < Actions.Count ; ++i) {
					if (Actions[i] == actionText) {
						Actions[i] = CancelText;
						return;
					}
				}
			}
		}

		string GetActionTextForState (GameUIState state)
		{
			switch (state)
			{
				case GameUIState.SelectRegionToBuildIn:
					return BuildText;
				case GameUIState.SelectBuildingToDestory:
					return DestroyText;
				default:
					return null;
			}
		}

		public void OnBuildAreaSelection (Area area)
		{
			var region = State.Regions.First (x => x.Areas.Contains (area));
			int areaIndex = region.Areas.IndexOf (area);

			var options = new Dictionary<string, object> {
				["Region"] = region,
				["AreaIndex"] = areaIndex
			};
			SetUIState (GameUIState.ShowBuildingSelectDialog, options);
		}

		public void OnSpecificBuildingSelection (string regionName, int areaIndex, string buildingName)
		{
			State = Engine.ApplyAction (State, BuildText, new string [] { regionName, areaIndex.ToString (), buildingName });
		}
		
		public void OnDestroySelection (Area area, int buildingPosition)
		{
			SetUIState (GameUIState.Default);
		
			var region = State.Regions.First (x => x.Areas.Contains (area));
			int areaIndex = region.Areas.IndexOf (area);
			State = Engine.ApplyAction (State, DestroyText, new string [] { region.Name, areaIndex.ToString (), buildingPosition.ToString () });
		}

		public void OnTick ()
		{
			State = Engine.ProcessTick (State);
		}
	}
}
