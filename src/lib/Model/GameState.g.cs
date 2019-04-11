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

		public Area (AreaType type)
		{
			Type = type;
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

	public partial class Resource
	{
		public string Name { get; }
		public int Amount { get; }

		public Resource (string name, int amount)
		{
			Name = name;
			Amount = amount;
		}
	}

	public partial class GameState
	{
		public Age Age { get; }
		public ImmutableArray<Region> Regions { get; }
		public ImmutableArray<Resource> Resources { get; }

		public GameState (Age age, IEnumerable<Region> regions, IEnumerable<Resource> resources)
		{
			Age = age;
			Regions = ImmutableArray.CreateRange (regions ?? Array.Empty<Region> ());
			Resources = ImmutableArray.CreateRange (resources ?? Array.Empty<Resource> ());
		}
	}
}
