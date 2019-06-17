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

	public enum RegionSize { Small, Medium, Large }

	public class RegionGenerator
	{
		NameGenerator NameGenerator;
		Dictionary<string, Ranges> ClimateRanges = new Dictionary<string, Ranges> ();
		Dictionary<string, double> ClimateFeatureChance = new Dictionary<string, double> ();

		Random Random;

		public RegionGenerator (JsonLoader json)
		{
			NameGenerator = new NameGenerator (json);
			Random = new Random ();
			foreach (var climate in json.Areas.Climates) {
				ClimateRanges[climate.Name] = new Ranges (climate.AreaChances.Select (x => (x.Name, x.Chance)));
				ClimateFeatureChance[climate.Name] = climate.FeatureChance * 100;
			}
		}

		public Area CreateArea (string climate)
		{
			string areaType = ClimateRanges[climate][Random.NextDouble ()];
			if (Random.WithChance (ClimateFeatureChance[climate)))
			{
				// Add a random feature for that area
			}

			return new Area (areaType);
		}

		int GetRegionCount (RegionSize size)
		{
			switch (size)
			{
				case RegionSize.Small:
					return Random.Next (3, 4);
				case RegionSize.Medium:
					return Random.Next (4, 5);
				case RegionSize.Large:
					return Random.Next (6, 7);
				default:
					throw new NotImplementedException ("Unknown RegionSize");
			}
		}

		public Region CreateRegion (RegionSize size, string climate)
		{
			var areas = GetRegionCount (size).Range ().Select (x => CreateArea (climate));
			var region = new Region (NameGenerator.Generate (), areas);
			return region;
		}
	}
}
