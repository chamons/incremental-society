using System.Collections.Generic;
using System.Linq;

namespace IncrementalSociety.Web.Services
{
	public class ActionService
	{
		GameService GameService;
		public List<string> Actions { get; private set; }

		GameUIState CurrentUIState => GameService.CurrentUIState;

		// These must match keys in GameEngine::ApplyAction
		public const string BuildText = "Build District";
		public const string DestroyText = "Destory District";
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
			Actions = new List <string> () { BuildText, DestroyText, NewGameText };
#if DEBUG
			Actions.Add ("Debug - Fill Resources");
#endif
		}

		void ReplaceActionWithCancel ()
		{
			string actionText = GetActionText ();
			if (actionText != null) {
				for (int i = 0; i < Actions.Count; ++i) {
					if (Actions[i] == actionText) {
						Actions[i] = CancelText;
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
				case CancelText:
					GameService.SetUIState (GameUIState.Default);
					return;
				case NewGameText:
					GameService.NewGame ();
					return;
				default:
					GameService.SetUIState (GameUIState.Default);
					GameService.ApplyAction (action);
					return;
			}
		}
	}
}
