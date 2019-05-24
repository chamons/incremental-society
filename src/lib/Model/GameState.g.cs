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
		public int Version { get; }
		public Age Age { get; }
		public ImmutableArray<Region> Regions { get; }
		public Resources Resources { get; }
		public double Population { get; }
		public double PopulationCap { get; }
		public ImmutableHashSet<string> ResearchUnlocks { get; }
		public ImmutableHashSet<string> DisabledConversions { get; }

		public GameState (int version, Age age, IEnumerable<Region> regions, Resources resources, double population, double populationCap, IEnumerable<string> researchUnlocks = null, IEnumerable<string> disabledConversions = null)
		{
			Version = version;
			Age = age;
			Regions = ImmutableArray.CreateRange (regions ?? Array.Empty<Region> ());
			Resources = resources;
			Population = population;
			PopulationCap = populationCap;
			ResearchUnlocks = ImmutableHashSet.CreateRange (researchUnlocks ?? Array.Empty<string> ());
			DisabledConversions = ImmutableHashSet.CreateRange (disabledConversions ?? Array.Empty<string> ());
		}

		public GameState WithVersion (int version)
		{
			return new GameState (version, Age, Regions, Resources, Population, PopulationCap, ResearchUnlocks, DisabledConversions);
		}

		public GameState WithAge (Age age)
		{
			return new GameState (Version, age, Regions, Resources, Population, PopulationCap, ResearchUnlocks, DisabledConversions);
		}

		public GameState WithRegions (IEnumerable<Region> regions)
		{
			return new GameState (Version, Age, regions, Resources, Population, PopulationCap, ResearchUnlocks, DisabledConversions);
		}

		public GameState WithResources (Resources resources)
		{
			return new GameState (Version, Age, Regions, resources, Population, PopulationCap, ResearchUnlocks, DisabledConversions);
		}

		public GameState WithPopulation (double population)
		{
			return new GameState (Version, Age, Regions, Resources, population, PopulationCap, ResearchUnlocks, DisabledConversions);
		}

		public GameState WithPopulationCap (double populationCap)
		{
			return new GameState (Version, Age, Regions, Resources, Population, populationCap, ResearchUnlocks, DisabledConversions);
		}

		public GameState WithResearchUnlocks (IEnumerable<string> researchUnlocks)
		{
			return new GameState (Version, Age, Regions, Resources, Population, PopulationCap, researchUnlocks, DisabledConversions);
		}

		public GameState WithDisabledConversions (IEnumerable<string> disabledConversions)
		{
			return new GameState (Version, Age, Regions, Resources, Population, PopulationCap, ResearchUnlocks, disabledConversions);
		}
	}

	public partial class ResearchItem
	{
		public string Name { get; }
		public string Description { get; }
		public bool IsResearched { get; }
		public Resources Cost { get; }

		public ResearchItem (string name, string description, bool isResearched, Resources cost)
		{
			Name = name;
			Description = description;
			IsResearched = isResearched;
			Cost = cost;
		}
	}
}
