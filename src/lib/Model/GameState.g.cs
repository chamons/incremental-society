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

		public Area WithType (AreaType type)
		{
			return new Area (type, Buildings);
		}

		public Area WithBuildings (IEnumerable<string> buildings)
		{
			return new Area (Type, buildings);
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

		public Region WithName (string name)
		{
			return new Region (name, Areas);
		}

		public Region WithAreas (IEnumerable<Area> areas)
		{
			return new Region (Name, areas);
		}
	}

	public partial class GameState
	{
		public Age Age { get; }
		public ImmutableArray<Region> Regions { get; }
		public ImmutableDictionary<string, double> Resources { get; }
		public double Population { get; }
		public double PopulationCap { get; }
		public ImmutableArray<string> DisabledConversions { get; }

		public GameState (Age age, IEnumerable<Region> regions, IDictionary<string, double> resources, double population, double populationCap, IEnumerable<string> disabledConversions = null)
		{
			Age = age;
			Regions = ImmutableArray.CreateRange (regions ?? Array.Empty<Region> ());
			Resources = resources.ToImmutableDictionary ();
			Population = population;
			PopulationCap = populationCap;
			DisabledConversions = ImmutableArray.CreateRange (disabledConversions ?? Array.Empty<string> ());
		}

		public GameState WithAge (Age age)
		{
			return new GameState (age, Regions, Resources, Population, PopulationCap, DisabledConversions);
		}

		public GameState WithRegions (IEnumerable<Region> regions)
		{
			return new GameState (Age, regions, Resources, Population, PopulationCap, DisabledConversions);
		}

		public GameState WithResources (IDictionary<string, double> resources)
		{
			return new GameState (Age, Regions, resources, Population, PopulationCap, DisabledConversions);
		}

		public GameState WithPopulation (double population)
		{
			return new GameState (Age, Regions, Resources, population, PopulationCap, DisabledConversions);
		}

		public GameState WithPopulationCap (double populationCap)
		{
			return new GameState (Age, Regions, Resources, Population, populationCap, DisabledConversions);
		}

		public GameState WithDisabledConversions (IEnumerable<string> disabledConversions)
		{
			return new GameState (Age, Regions, Resources, Population, PopulationCap, disabledConversions);
		}
	}
}
