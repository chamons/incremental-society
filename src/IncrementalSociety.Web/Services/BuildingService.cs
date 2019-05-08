using System.Collections.Generic;
using System.Linq;

using IncrementalSociety.Model;

namespace IncrementalSociety.Web.Services
{
	public class BuildingService
	{
		GameService GameService;
		GameState State => GameService.State;

		public BuildingService (GameService gameService)
		{
			GameService = gameService;
		}

		public void OnBuildAreaSelection (Area area)
		{
			var region = State.Regions.First (x => x.Areas.Contains (area));
			int areaIndex = region.Areas.IndexOf (area);

			var options = new Dictionary<string, object> {
				["Region"] = region,
				["AreaIndex"] = areaIndex
			};
			GameService.SetUIState (GameUIState.ShowBuildingSelectDialog, options);
		}

		public void OnSpecificBuildingSelection (string regionName, int areaIndex, string buildingName)
		{
			GameService.ApplyAction (ActionService.BuildText, new string[] { regionName, areaIndex.ToString (), buildingName });
		}

		public void OnDestroySelection (Area area, int buildingPosition)
		{
			GameService.SetUIState (GameUIState.Default);

			var region = State.Regions.First (x => x.Areas.Contains (area));
			int areaIndex = region.Areas.IndexOf (area);
			GameService.ApplyAction (ActionService.DestroyText, new string[] { region.Name, areaIndex.ToString (), buildingPosition.ToString () });
		}

	}
}
