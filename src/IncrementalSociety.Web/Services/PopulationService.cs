using System.Collections.Generic;
using System.Linq;

using IncrementalSociety.Model;

namespace IncrementalSociety.Web.Services
{
	public class PopulationService
	{
		GameService GameService;
		GameState State => GameService.State;

		public PopulationService (GameService gameService)
		{
			GameService = gameService;
		}

		public int BuildingJobCount => GameService.Engine.GetBuildingJobCount (State);
		public double MaxBuildings => GameService.Engine.GetMaxBuildings (State);
		public double CurrentHousing => GameService.Engine.GetHousingCapacity (State);
		public double Efficiency => GameService.Engine.GetEfficiency (State);
		public double EffectiveCap => GameService.Engine.FindEffectivePopulationCap (State);

		public bool CanDecrement => GameService.Engine.CanDecreasePopulationCap (State);
		public bool CanIncrement => GameService.Engine.CanIncreasePopulationCap (State);
		public string CapDecrementAmount {
			get {
				double amount = GameService.Engine.GetPopCapDecrementAmount (State);
				return amount == 0 ? "-" : amount.ToString ();
			}
		}
		public string CapIncrementAmount => "+" + GameService.Engine.GetPopCapIncrementAmount (State);
		public bool IsPopulationStarving => GameService.Engine.IsPopulationStarving (State);
	}
}
