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
		PopUnits PopUnits;

		public PopulationBuildingInfo (ResourceEngine resourceEngine, PopUnits popUnits)
		{
			ResourceEngine = resourceEngine;
			PopUnits = popUnits;
		}

		public int GetBuildingJobCount (GameState state)
		{
			return state.AllBuildings ().Where (x => !ResourceEngine.FindBuilding (x).DoesNotRequireJob).Count ();
		}

		public double GetPopulationEfficiency (GameState state)
		{
			int buildingCount = GetBuildingJobCount (state);
			double totalPopCount = PopUnits.GetPopUnitsForTotalPopulation (state.Population);
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