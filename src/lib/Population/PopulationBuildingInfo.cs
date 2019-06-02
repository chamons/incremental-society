using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Text;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

namespace IncrementalSociety.Population
{
	public class PopulationBuildingInfo
	{
		ResourceEngine ResourceEngine;
		PopulationCapacity PopulationCapacity;

		public PopulationBuildingInfo (ResourceEngine resourceEngine, PopulationCapacity populationCapacity)
		{
			ResourceEngine = resourceEngine;
			PopulationCapacity = populationCapacity;
		}

		public int GetBuildingJobCount (GameState state)
		{
			return state.AllBuildings ().Where (x => !ResourceEngine.FindBuilding (x).DoesNotRequireJob).Count ();
		}

		public double GetPopulationEfficiency (GameState state)
		{
			int buildingCount = GetBuildingJobCount (state);
			double totalPopCount = PopulationCapacity.GetPopUnitsForTotalPopulation (state.Population);
			return GetPopulationEfficiency (buildingCount, totalPopCount);
		}

		public double GetPopulationEfficiency (int buildingCount, double totalPopCount)
		{
			if (totalPopCount >= buildingCount)
				return 1.0;
			else
				return 1.0 - ((double)(buildingCount - totalPopCount) / (double)buildingCount);
		}
	}
}