using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using Newtonsoft.Json;

using IncrementalSociety.Population;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;
using IncrementalSociety.Generator;

namespace IncrementalSociety
{
	public class GameEngine
	{
		PopulationEngine PopulationEngine;
		ResourceEngine ResourceEngine;
		BuildingEngine BuildingEngine;
		ResearchEngine ResearchEngine;
		EdictsEngine EdictsEngine;
		PopulationBuildingInfo PopulationBuildingInfo;
		PopulationCapacity PopulationCapacity;
		PopulationResources PopulationResources;
		PopulationNeeds PopulationNeeds;
		PopUnits PopUnits;
		JsonLoader Json;

		public static GameEngine Create (JsonLoader loader)
		{
			return new GameEngine (loader, new ResourceEngine (loader));
		}

		GameEngine (JsonLoader loader, ResourceEngine resourceEngine)
		{
			Json = loader;
			ResourceEngine = resourceEngine;
			PopUnits = new PopUnits (loader.Game.MinPopulation);
			PopulationBuildingInfo = new PopulationBuildingInfo (ResourceEngine, PopUnits);
			ResearchEngine = new ResearchEngine (ResourceEngine, loader);
			EdictsEngine = new EdictsEngine (ResourceEngine, loader);
			PopulationResources = new PopulationResources (ResourceEngine, PopulationBuildingInfo, loader);

			PopulationCapacity = new PopulationCapacity (ResourceEngine, PopulationResources, PopulationBuildingInfo, PopUnits);
			PopulationNeeds = new PopulationNeeds (ResourceEngine, loader, PopUnits, PopulationResources);
			PopulationEngine = new PopulationEngine (ResourceEngine, PopulationCapacity, PopulationResources, PopulationNeeds, loader);
			BuildingEngine = new BuildingEngine (ResourceEngine, PopulationEngine);
		}

		public void ConfigureForLoad ()
		{
			// So when we load, we do not have sufficient state to inflate the
			// resource lists, as we do not serialize the index.
			// There is no need, as they must match our json
			// So apply a bit of hacky static state
			Resources.SaveLoadConfig = ResourceEngine.ResourceConfig;
			EdictCooldown.SaveLoadConfig = EdictsEngine.EdictConfig;
		}

		public GameState ApplyAction (GameState state, string action, string [] args = null)
		{
			switch (action)
			{
				case "Grow Population Cap":
					state = PopulationCapacity.IncreasePopulationCap (state);
					break;
				case "Lower Population Cap":
					state = PopulationCapacity.DecreasePopulationCap (state);
					break;
				case "Build District":
				{
					string regionName = args[0];
					int regionIndex = int.Parse (args[1]);
					string buildingName = args[2];
					state = BuildingEngine.Build (state, regionName, regionIndex, buildingName);
					break;
				}
				case "Destory District":
				{
					string regionName = args[0];
					int regionIndex = int.Parse (args[1]);
					int buildingIndex = int.Parse (args[2]);
					state = BuildingEngine.Destroy (state, regionName, regionIndex, buildingIndex);
					break;
				}
				case "Research":
				{
					string techName = args[0];
					int selection = args.Length > 1 ? int.Parse (args[1]) : -1;
					state = ResearchEngine.Research (state, techName, selection);
					break;
				}
				case "Edict":
				{
					string edictName = args[0];
					state = EdictsEngine.ApplyEdict (state, edictName);
					break;
				}
#if DEBUG
				case "Debug - Fill Resources":
				{
					state = state.WithResources (ResourceEngine.GetResourceStorage (state));
					break;
				}
				case "Debug - Fill Population":
				{
					state = state.WithPopulation (state.PopulationCap);
					break;
				}
#endif
				default:
					throw new InvalidOperationException ($"Unable to find action {action}");
			}
			return state;
		}

		public bool IsConversionEnabled (GameState state, string name) => ResourceEngine.IsConversionEnabled (state, name);

		public GameState ToggleConversion (GameState state, string conversion)
		{
			if (IsConversionEnabled (state, conversion))
				return state.WithDisabledConversions (state.DisabledConversions.Add (conversion));
			else
				return state.WithDisabledConversions (state.DisabledConversions.Remove (conversion));
		}

		public List<(string Name, bool Enabled)> GetConversions (GameState state) => ResourceEngine.GetConversions (state);

		public List<(string Name, Resources Resources)> GetBuildingConversionResources (GameState state, string name)
		{
			return ResourceEngine.GetBuildingConversionResources (state, name);
		}

		public double GetEfficiency (GameState state) => PopulationBuildingInfo.GetPopulationEfficiency (state);

		public double FindEffectivePopulationCap (GameState state) => PopulationCapacity.FindEffectiveCap (state);

		public GameState ProcessTick (GameState state)
		{
			state = ResourceEngine.AddTickOfResources (state, GetEfficiency (state));
			state = PopulationEngine.ProcessTick (state);
			state = ResourceEngine.ConstrainResourcesToStorage (state);
			state = EdictsEngine.ProcessTick (state);
			return state;
		}

		public List<string> GetValidBuildingsForArea (GameState state, Area area) => BuildingEngine.GetValidBuildingsForArea (state, area);

		public bool CanAffordBuilding (GameState state, string buildingName) => BuildingEngine.CanAffordBuilding (state, buildingName);
		public bool AbleToBuild (string buildingName) => !ResourceEngine.FindBuilding (buildingName).PreventBuild;

		public Resources GetResourcesNextTick (GameState state)
		{
			var nextTickResources = ResourceEngine.CalculateAdditionalNextTick (state, GetEfficiency (state)).ToBuilder ();
			nextTickResources.Subtract (PopulationResources.GetRequirementsForCurrentPopulation (state));
			return nextTickResources.ToResources ();
		}

		public int GetRegionCapacity (GameState state) => ResourceEngine.GetRegionCapacity (state);
		public Resources GetAreaBonusResources (Area area) => ResourceEngine.GetAreaBonus (area);

		public Resources GetBuildingResources (GameState state, string building) => ResourceEngine.GetBuildingResources (state, building);
		public Resources GetBuildingCost (GameState state, string building) => ResourceEngine.GetBuildingCost (state, building);
		public Resources GetBuildingStorage (GameState state, string building) => ResourceEngine.GetBuildingStorage (state, building);
		public double GetBuildingHousing (GameState state, string building) => ResourceEngine.GetBuildingHousing (state, building);

		public bool CanDestoryBuilding (string buildingName) => !ResourceEngine.FindBuilding (buildingName).PreventDestroy;

		public int GetBuildingJobCount (GameState state) => PopulationBuildingInfo.GetBuildingJobCount (state);
		public int GetBuildingTotal (GameState state) => state.AllBuildings ().Count ();
		public double GetMaxBuildings (GameState state) => PopUnits.GetPopUnitsForTotalPopulation (state.Population);
		public double GetHousingCapacity (GameState state) => PopulationCapacity.GetHousingCapacity (state);

		public bool CanIncreasePopulationCap (GameState state) => PopulationCapacity.CanIncreasePopulationCap (state);
		public bool CanDecreasePopulationCap (GameState state) => PopulationCapacity.CanDecreasePopulationCap (state);
		public double GetPopCapDecrementAmount (GameState state) => PopUnits.GetPreviousPopBreakpoint (state.PopulationCap) - state.PopulationCap;
		public double GetPopCapIncrementAmount (GameState state) => PopUnits.GetNextPopBreakpoint (state.PopulationCap) - state.PopulationCap;
		public bool IsPopulationStarving (GameState state) => PopulationResources.IsPopulationStarving (state);

		public double GetHealth (GameState state) => PopulationNeeds.CalculateHealth (state).Value;
		public double GetHappiness (GameState state) => PopulationNeeds.CalculateHappiness (state).Value;
		public double GetGrowthRate (GameState state) => PopulationEngine.CalculateGrowthRate (state);

		public (double PopGrowth, double Immigration, double Emmigration, double Death) GetGrowthComponents (GameState state)
		{
			return PopulationEngine.GetGrowthComponents (state);
		}

		public Resources GetResourceStorage (GameState state) => ResourceEngine.GetResourceStorage (state);

		public List<ResearchItem> GetCurrentResearchOptions (GameState state) => ResearchEngine.GetCurrentResearchOptions (state);
		public bool CanResearch (GameState state, string techName, int specialization = -1) => ResearchEngine.CanResearch (state, techName, specialization);
		public List<ResearchItem> GetResearchSpecializations (string techName) => ResearchEngine.GetResearchSpecializations (techName);
 		public IEnumerable<(string Name, bool CanApply)> AvailableEdicts (GameState state) => EdictsEngine.AvailableEdicts (state);

		public const int CurrentVersion = 1;

		Region PlaceSettlement (Region region)
		{
			var firstArea = region.Areas[0];
			var newAreas = region.Areas.Replace (firstArea, firstArea.WithBuildings ("Crude Settlement".Yield ()));
			return region.WithAreas (newAreas);
		}

		public GameState CreateNewGame ()
		{
			var generator = new RegionGenerator (Json);
			var region = generator.CreateRegion (RegionSize.Large, "Standard");
			region = PlaceSettlement (region);

			var resources = ResourceEngine.ResourceConfig.CreateBuilder ();
			resources["Food"] = 50;
			resources["Wood"] = 50;
			return new GameState (CurrentVersion, "Stone", new Region[] { region }, resources, 200, 200, EdictsEngine.EdictConfig.Create ());
		}
	}
}
