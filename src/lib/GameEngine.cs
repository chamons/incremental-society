using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;
using Newtonsoft.Json;

namespace IncrementalSociety
{
	public class GameEngine
	{
		PopulationEngine PopulationEngine;
		ResourceEngine ResourceEngine;
		BuildingEngine BuildingEngine;
		ResearchEngine ResearchEngine;
		EdictsEngine EdictsEngine;

		public static GameEngine Create (JsonLoader loader)
		{
			return new GameEngine (loader, new ResourceEngine (loader));
		}

		public GameEngine (JsonLoader loader, ResourceEngine resourceEngine)
		{
			ResourceEngine = resourceEngine;
			PopulationEngine = new PopulationEngine (ResourceEngine, loader);
			BuildingEngine = new BuildingEngine (ResourceEngine, PopulationEngine);
			ResearchEngine = new ResearchEngine (ResourceEngine, loader);
			EdictsEngine = new EdictsEngine (ResourceEngine, loader);
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
					state = PopulationEngine.IncreasePopulationCap (state);
					break;
				case "Lower Population Cap":
					state = PopulationEngine.DecreasePopulationCap (state);
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
					state = ResearchEngine.Research (state, techName);
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

		public double GetEfficiencyOfNonBasicGoods (GameState state)
		{
			if (IsPopulationStarving (state))
				return 0;
			return PopulationEngine.GetPopulationEfficiency (state);
		}

		public double FindEffectivePopulationCap (GameState state) => PopulationEngine.FindEffectiveCap (state);

		public GameState ProcessTick (GameState state)
		{
			state = ResourceEngine.AddTickOfResources (state, GetEfficiencyOfNonBasicGoods (state));
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
			var nextTickResources = ResourceEngine.CalculateAdditionalNextTick (state, GetEfficiencyOfNonBasicGoods (state)).ToBuilder ();
			nextTickResources.Subtract (PopulationEngine.GetRequirementsForCurrentPopulation (state));
			return nextTickResources.ToResources ();
		}

		public int GetRegionCapacity (GameState state) => ResourceEngine.GetRegionCapacity (state);

		public Resources GetBuildingResources (GameState state, string building) => ResourceEngine.GetBuildingResources (state, building);
		public Resources GetBuildingCost (GameState state, string building) => ResourceEngine.GetBuildingCost (state, building);
		public Resources GetBuildingStorage (GameState state, string building) => ResourceEngine.GetBuildingStorage (state, building);
		public double GetBuildingHousing (GameState state, string building) => ResourceEngine.GetBuildingHousing (state, building);

		public bool CanDestoryBuilding (string buildingName) => !ResourceEngine.FindBuilding (buildingName).PreventDestroy;

		public int GetBuildingJobCount (GameState state) => PopulationEngine.GetBuildingJobCount (state);
		public int GetBuildingTotal (GameState state) => state.AllBuildings ().Count ();
		public double GetMaxBuildings (GameState state) => PopulationEngine.GetPopUnitsForTotalPopulation (state.Population);
		public double GetHousingCapacity (GameState state) => PopulationEngine.GetHousingCapacity (state);

		public bool CanIncreasePopulationCap (GameState state) => PopulationEngine.CanIncreasePopulationCap (state);
		public bool CanDecreasePopulationCap (GameState state) => PopulationEngine.CanDecreasePopulationCap (state);
		public double GetPopCapDecrementAmount (GameState state) => PopulationEngine.GetPreviousPopBreakpoint (state.PopulationCap) - state.PopulationCap;
		public double GetPopCapIncrementAmount (GameState state) => PopulationEngine.GetNextPopBreakpoint (state.PopulationCap) - state.PopulationCap;
		public bool IsPopulationStarving (GameState state) => PopulationEngine.IsPopulationStarving (state);

		public Resources GetResourceStorage (GameState state) => ResourceEngine.GetResourceStorage (state);

		public List<ResearchItem> GetCurrentResearchOptions (GameState state) => ResearchEngine.GetCurrentResearchOptions (state);
		public bool CanResearch (GameState state, string techName) => ResearchEngine.CanResearch (state, techName);
		public IEnumerable<(string Name, bool CanApply)> AvailableEdicts (GameState state) => EdictsEngine.AvailableEdicts (state);

		public const int CurrentVersion = 1;

		public GameState CreateNewGame ()
		{
			var greenlandRegion = new Region ("Greenland", new Area[] { new Area (AreaType.Forest, new string[] { "Crude Settlement" }), new Area (AreaType.Plains), new Area (AreaType.Forest), new Area (AreaType.Forest), new Area (AreaType.Ocean) });
			var resources = ResourceEngine.ResourceConfig.CreateBuilder ();
			resources["Food"] = 50;
			resources["Wood"] = 50;
			return new GameState (CurrentVersion, Age.Stone, new Region[] { greenlandRegion }, resources, 200, 200, EdictsEngine.EdictConfig.Create ());
		}
	}
}
