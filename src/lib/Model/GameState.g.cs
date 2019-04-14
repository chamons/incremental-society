using System;
using System.Collections.Generic;
using System.Collections.Immutable;

namespace IncrementalSociety.Model
{
	public enum AreaType
	{
		Forest,
		Plains,
		Mountain,
		Ocean,
		Swamp,
		Desert
	}

	public enum Age
	{
		Stone,
		Bronze
	}

	public partial class Area
	{
		public AreaType Type { get; }
		public ImmutableArray<string> Buildings { get; }

		public Area (AreaType type, IEnumerable<string> buildings = null)
		{
			Type = type;
			Buildings = ImmutableArray.CreateRange (buildings ?? Array.Empty<string> ());
		}
	}

	public partial class Region
	{
		public string Name { get; }
		public ImmutableArray<Area> Areas { get; }

		public Region (string name, IEnumerable<Area> areas)
		{
			Name = name;
			Areas = ImmutableArray.CreateRange (areas ?? Array.Empty<Area> ());
		}
	}

	public partial class GameState
	{
		public Age Age { get; }
		public ImmutableArray<Region> Regions { get; }
		public ImmutableDictionary<string, int> Resources { get; }

		public GameState (Age age, IEnumerable<Region> regions, Dictionary<string, int> resources)
		{
			Age = age;
			Regions = ImmutableArray.CreateRange (regions ?? Array.Empty<Region> ());
			Resources = resources.ToImmutableDictionary ();
		}
	}
}
