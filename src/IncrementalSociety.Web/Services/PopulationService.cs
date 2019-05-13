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

		public int CurrentBuildings => GameService.Engine.GetBuildingTotal (State);
		public int MaxBuildings => GameService.Engine.GetMaxBuildings (State);
		public double CurrentHousing => GameService.Engine.GetHousingCapacity (State);

		public bool CanDecrement => GameService.Engine.CanDecreasePopulationCap (State);
		public bool CanIncrement => GameService.Engine.CanIncreasePopulationCap (State);
		public string CapDecrementAmount {
			get {
				double amount = GameService.Engine.GetPopCapDecrementAmount (State);
				return amount == 0 ? "-" : amount.ToString (); 
			}
		}
		public string CapIncrementAmount => "+" + GameService.Engine.GetPopCapIncrementAmount (State);
	}
}
