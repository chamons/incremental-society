using System;
using System.Collections.Generic;
using System.Collections.Immutable;

namespace IncrementalSociety.Model
{
	public partial class Area
	{
		public string Type { get; }
		public ImmutableArray<string> Features { get; }
		public ImmutableArray<string> Buildings { get; }

		public Area (string type, IEnumerable<string> features = null, IEnumerable<string> buildings = null)
		{
			Type = type;
			Features = ImmutableArray.CreateRange (features ?? Array.Empty<string> ());
			Buildings = ImmutableArray.CreateRange (buildings ?? Array.Empty<string> ());
		}

		public Area WithType (string type)
		{
			return new Area (type, Features, Buildings);
		}

		public Area WithFeatures (IEnumerable<string> features)
		{
			return new Area (Type, features, Buildings);
		}

		public Area WithBuildings (IEnumerable<string> buildings)
		{
			return new Area (Type, Features, buildings);
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
		public string Age { get; }
		public ImmutableArray<Region> Regions { get; }
		public Resources Resources { get; }
		public double Population { get; }
		public double PopulationCap { get; }
		public EdictCooldown Edicts { get; }
		public ImmutableHashSet<string> ResearchUnlocks { get; }
		public ImmutableHashSet<string> DisabledConversions { get; }

		public GameState (int version, string age, IEnumerable<Region> regions, Resources resources, double population, double populationCap, EdictCooldown edicts, IEnumerable<string> researchUnlocks = null, IEnumerable<string> disabledConversions = null)
		{
			Version = version;
			Age = age;
			Regions = ImmutableArray.CreateRange (regions ?? Array.Empty<Region> ());
			Resources = resources;
			Population = population;
			PopulationCap = populationCap;
			Edicts = edicts;
			ResearchUnlocks = ImmutableHashSet.CreateRange (researchUnlocks ?? Array.Empty<string> ());
			DisabledConversions = ImmutableHashSet.CreateRange (disabledConversions ?? Array.Empty<string> ());
		}

		public GameState WithVersion (int version)
		{
			return new GameState (version, Age, Regions, Resources, Population, PopulationCap, Edicts, ResearchUnlocks, DisabledConversions);
		}

		public GameState WithAge (string age)
		{
			return new GameState (Version, age, Regions, Resources, Population, PopulationCap, Edicts, ResearchUnlocks, DisabledConversions);
		}

		public GameState WithRegions (IEnumerable<Region> regions)
		{
			return new GameState (Version, Age, regions, Resources, Population, PopulationCap, Edicts, ResearchUnlocks, DisabledConversions);
		}

		public GameState WithResources (Resources resources)
		{
			return new GameState (Version, Age, Regions, resources, Population, PopulationCap, Edicts, ResearchUnlocks, DisabledConversions);
		}

		public GameState WithPopulation (double population)
		{
			return new GameState (Version, Age, Regions, Resources, population, PopulationCap, Edicts, ResearchUnlocks, DisabledConversions);
		}

		public GameState WithPopulationCap (double populationCap)
		{
			return new GameState (Version, Age, Regions, Resources, Population, populationCap, Edicts, ResearchUnlocks, DisabledConversions);
		}

		public GameState WithEdicts (EdictCooldown edicts)
		{
			return new GameState (Version, Age, Regions, Resources, Population, PopulationCap, edicts, ResearchUnlocks, DisabledConversions);
		}

		public GameState WithResearchUnlocks (IEnumerable<string> researchUnlocks)
		{
			return new GameState (Version, Age, Regions, Resources, Population, PopulationCap, Edicts, researchUnlocks, DisabledConversions);
		}

		public GameState WithDisabledConversions (IEnumerable<string> disabledConversions)
		{
			return new GameState (Version, Age, Regions, Resources, Population, PopulationCap, Edicts, ResearchUnlocks, disabledConversions);
		}

		public bool HasResearch (string tech) => tech == null || ResearchUnlocks.Contains (tech);
	}

	public partial class ResearchItem
	{
		public string Name { get; }
		public string Description { get; }
		public Resources Cost { get; }

		public ResearchItem (string name, string description, Resources cost)
		{
			Name = name;
			Description = description;
			Cost = cost;
		}
	}
}
