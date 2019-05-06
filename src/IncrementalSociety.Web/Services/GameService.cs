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
		JsonLoader Loader;
		GameEngine Engine;
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

		public string GetImageFilename (string name)
		{
			return GetImageFilename (Resources.First (x => x.Name == name));
		}

		public string GetImageFilename (ResourceDeclaration decl)
		{
			string name = decl.Name.ToLower ().Replace (' ', '-');
			if (decl.ImageHasAgePrefix)
				return $"images\\{State.Age}-{name}.png";
			else
				return $"images\\{name}.png";
		}

		public ImmutableDictionary<string, double> GetBuildingResources (string building)
		{
			return Engine.GetBuildingResources (building);
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

		public void SetUIState (GameUIState state, Dictionary<string, object> options = null)
		{
			Console.Error.WriteLine ($"SetUIState: {state}");
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

			// State = Engine.ApplyAction (State, BuildText, new string [] { region.Name, areaIndex.ToString (), "Gathering Camp" });
		
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

		public string GetResourceDeltaClass (double count)
		{
			if (count < -.001) {
				return "red";
			}
			if (count > .001) {
				return "green";
			}
			return "clear";
		}
	}
}
