using System;
using System.Collections.Generic;
using System.Linq;

using IncrementalSociety.Model;
using IncrementalSociety.Json;
using System.Collections.Immutable;

namespace IncrementalSociety.Web.Services
{
	public enum GameUIState { Default, SelectForBuild, SelectForDestory }

	public class GameService
	{
		JsonLoader Loader;
		GameEngine Engine;
		public Action NotifyUIStateHasChanged;

		public GameUIState CurrentUIState { get; private set; } = GameUIState.Default;
		public GameState State { get; private set; }
		public int RegionCapacity { get; private set; }
	
		const string CancelText = "Cancel";
		const string BuildText = "Build District";
		const string DestroyText = "Destory District";

		public GameService ()
		{
			Loader = JsonLoader.Load ();
			RegionCapacity = Loader.Game.RegionCapacity;
			State = GameEngine.CreateNewGame ();
			Engine = new GameEngine ();
			
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
			NotifyUIStateHasChanged ();
			switch (action)
			{
				case BuildText:
					SetUIState (GameUIState.SelectForBuild);
					return;
				case DestroyText:
					SetUIState (GameUIState.SelectForDestory);
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

		void SetUIState (GameUIState state)
		{
			ResetActionList ();
			CurrentUIState = state;
			if (state != GameUIState.Default)
				ReplaceActionWithCancel (state);
		}

		void ReplaceActionWithCancel (GameUIState state)
		{
			string actionText = GetActionTextForState (state);
			for (int i = 0 ; i < Actions.Count ; ++i) {
				if (Actions[i] == actionText) {
					Actions[i] = CancelText;
					return;
				}
			}
		}

		string GetActionTextForState (GameUIState state)
		{
			switch (state)
			{
				case GameUIState.SelectForBuild:
					return BuildText;
				case GameUIState.SelectForDestory:
					return DestroyText;
			}
			throw new InvalidOperationException ($"GetActionTextForState with state {state}");
		}

		public void OnBuildSelection (Region r, int position)
		{
			SetUIState (GameUIState.Default);
			NotifyUIStateHasChanged ();
		}
		
		public void OnDestroySelection (Region r, int position)
		{
			SetUIState (GameUIState.Default);
			NotifyUIStateHasChanged ();
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
