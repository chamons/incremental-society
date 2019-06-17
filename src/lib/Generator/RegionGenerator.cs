using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

using IncrementalSociety.Population;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

namespace IncrementalSociety.Generator
{
	public class Ranges
	{
		string [] Names;
		double [] Values;

		public Ranges (IEnumerable<(string Name, double Value)> items)
		{
			Names = items.Select (x => x.Name).ToArray ();
			Values = items.Select (x => x.Value).ToArray ();

			if (!Values.Sum ().Is (1.0))
				throw new InvalidOperationException ("Values to Range did not add up to 1.0");
		}

		public string this[double index]
		{
			get
			{
				if (index < 0 || index > 1)
					throw new InvalidOperationException ($"Ranges index given invalid index of {index}");
				double currentOffset = 0;
				for (int i = 0 ; i < Values.Length; ++i) {
					if (currentOffset + Values[i] >= index)
						return Names[i];
					currentOffset += Values[i];
				}
				throw new InvalidOperationException ($"Unable to find value {index} in Range");
			}
		}
	}

	public class RegionGenerator
	{
		JsonLoader Json;
		Dictionary<string, Ranges> ClimateRanges = new Dictionary<string, Ranges> ();
		Random Random;

		public RegionGenerator (JsonLoader json)
		{
			Json = json;
			Random = new Random ();
			foreach (var climate in json.Areas.Climates)
				ClimateRanges [climate.Name] = new Ranges (climate.AreaChances.Select (x => (x.Name, x.Chance)));
		}

		public Area CreateArea (string climate)
		{
			double r = Random.NextDouble ();
			string areaType = ClimateRanges[climate][r];
			return new Area (areaType);
		}
	}
}
