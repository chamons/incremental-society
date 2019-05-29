using System.Collections.Generic;
using System.Linq;

namespace IncrementalSociety.Web.Services
{
	public class ActionService
	{
		GameService GameService;
		public List<(string Name, bool Enabled)> Actions { get; private set; }

		GameUIState CurrentUIState => GameService.CurrentUIState;

		// These must match keys in GameEngine::ApplyAction
		public const string BuildText = "Build District";
		public const string DestroyText = "Destory District";
		public const string ResearchText = "Research";
		public const string NewGameText = "New Game";
		public const string CancelText = "Cancel";

		public ActionService (GameService gameService)
		{
			GameService = gameService;
			GameService.CurrentUIStateChanged += (o, e) => CurrentUIStateChanged ();

			ResetActionList ();
		}

		void CurrentUIStateChanged ()
		{
			ResetActionList ();
			ReplaceActionWithCancel ();
		}

		void ResetActionList ()
		{
			var edicts = GameService.Engine.AvailableEdicts (GameService.State).ToList ();
			Actions = new List <(string Name, bool Enabled)> (edicts.Count + 4) { (BuildText, true), (DestroyText, true), (ResearchText, true), (NewGameText, true) };
			foreach (var edict in GameService.Engine.AvailableEdicts (GameService.State))
				Actions.Add ((edict.Name, edict.Cooldown == 0));
#if DEBUG
			Actions.Add (("Debug - Fill Resources", true));
			Actions.Add (("Debug - Fill Population", true));
#endif
		}

		void ReplaceActionWithCancel ()
		{
			string actionText = GetActionText ();
			if (actionText != null) {
				for (int i = 0; i < Actions.Count; ++i) {
					if (Actions[i].Name == actionText) {
						Actions[i] = (CancelText, true);
						return;
					}
				}
			}
		}

		string GetActionText ()
		{
			switch (CurrentUIState)
			{
				case GameUIState.SelectRegionToBuildIn:
					return BuildText;
				case GameUIState.SelectBuildingToDestory:
					return DestroyText;
				default:
					return null;
			}
		}

		public void ApplyAction (string action)
		{
			switch (action)
			{
				case BuildText:
					GameService.SetUIState (GameUIState.SelectRegionToBuildIn);
					return;
				case DestroyText:
					GameService.SetUIState (GameUIState.SelectBuildingToDestory);
					return;
				case ResearchText:
					GameService.SetUIState (GameUIState.ShowResearchSelectDialog);
					return;
				case CancelText:
					GameService.SetUIState (GameUIState.Default);
					return;
				case NewGameText:
					GameService.NewGame ();
					return;
				default:
					GameService.SetUIState (GameUIState.Default);
					if (GameService.Engine.AvailableEdicts (GameService.State).Any (x => x.Name == action))
						GameService.ApplyAction ("Edict", new string[] { action });
					else
						GameService.ApplyAction (action);
					return;
			}
		}
	}
}
