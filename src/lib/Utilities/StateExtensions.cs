using System;
using System.Collections.Generic;
using System.Text;
using IncrementalSociety.Model;

namespace IncrementalSociety.Utilities
{
	static class StateExtensions
	{
		public static IEnumerable<string> AllBuildings (this GameState state)
		{
			foreach (var region in state.Regions)
				foreach (var area in region.Areas)
					foreach (var building in area.Buildings)
						yield return building;
		}
	}
}
