using System;
using System.Collections.Generic;
using System.Text;
using Newtonsoft.Json;

namespace IncrementalSociety.Json
{
	public class CapacityDeclaration
	{
		[JsonProperty ("capacity")]
		public int Capacity { get; set; }

		[JsonProperty ("required_technology", NullValueHandling = NullValueHandling.Ignore)]
		public string RequireTechnology { get; set; }
	}

	public class ResourceDeclaration
	{
		[JsonProperty ("name")]
		public string Name { get; set; }

		[JsonProperty ("Image_has_age_prefix")]
		public bool ImageHasAgePrefix { get; set; }
	}

	public class ResourceDeclarations
	{
		public List<ResourceDeclaration> Resources { get; set; }
	}

	public class RegionDeclaration
	{
		[JsonProperty ("name")]
		public string Name { get; set; }
		[JsonProperty ("resources")]
		public List<string> Resources { get; set; }
		[JsonProperty ("can_spawn")]
		public bool? CanSpawn { get; set; }
	}

	public class RegionDeclarations
	{
		public List<RegionDeclaration> Regions { get; set; }
	}

	public class GameDeclarations
	{
		[JsonProperty ("population_needs")]
		public Yield[] PopulationNeeds { get; set; }

		[JsonProperty ("region_capacity")]
		public CapacityDeclaration [] RegionCapacityDeclarations { get; set; }

		[JsonProperty ("min_population")]
		public int MinPopulation { get; set; }
	}

	public partial class BuildingDeclarations
	{
		[JsonProperty ("buildings")]
		public Building[] Buildings { get; set; }
	}

	public partial class Building
	{
		[JsonProperty ("name")]
		public string Name { get; set; }

		[JsonProperty ("prevent_build")]
		public bool PreventBuild { get; set; }

		[JsonProperty ("prevent_destory")]
		public bool PreventDestroy { get; set; }

		[JsonProperty ("does_not_require_job")]
		public bool DoesNotRequireJob { get; set; }

		[JsonProperty ("valid_regions")]
		public string[] ValidRegions { get; set; }

		[JsonProperty ("yield", NullValueHandling = NullValueHandling.Ignore)]
		public Yield[] Yield { get; set; }

		[JsonProperty ("conversion_yield", NullValueHandling = NullValueHandling.Ignore)]
		public ConversionYield[] ConversionYield { get; set; }

		[JsonProperty ("RequiredResource", NullValueHandling = NullValueHandling.Ignore)]
		public string RequiredResource { get; set; }

		[JsonProperty ("storage", NullValueHandling = NullValueHandling.Ignore)]
		public Yield[] Storage { get; set; }

		[JsonProperty ("cost")]
		public Yield[] Cost { get; set; }

		[JsonProperty ("housing_capacity")]
		public CapacityDeclaration [] HousingCapacity { get; set; }

		[JsonProperty ("required_technology", NullValueHandling = NullValueHandling.Ignore)]
		public string RequireTechnology { get; set; }
	}

	public partial class ConversionYield
	{
		[JsonProperty ("cost")]
		public Yield[] Cost { get; set; }

		[JsonProperty ("provides")]
		public Yield[] Provides { get; set; }

		[JsonProperty ("name", NullValueHandling = NullValueHandling.Ignore)]
		public string Name { get; set; }
	}

	public partial class Yield
	{
		[JsonProperty ("name", NullValueHandling = NullValueHandling.Ignore)]
		public string Name { get; set; }

		[JsonProperty ("amount")]
		public double Amount { get; set; }

		[JsonProperty ("required_technology", NullValueHandling = NullValueHandling.Ignore)]
		public string RequireTechnology { get; set; }
	}

	public class ResearchDeclaration
	{
		[JsonProperty ("name")]
		public string Name { get; set; }

		[JsonProperty ("description")]
		public string Description { get; set; }

		[JsonProperty ("dependencies")]
		public List<string> Dependencies { get; set; }

		[JsonProperty ("cost")]
		public Yield[] Cost { get; set; }

		[JsonProperty ("specializations")]
		public string[] Specializations { get; set; }
	}

	public class ResearchDeclarations
	{
		[JsonProperty ("research")]
		public List<ResearchDeclaration> Research { get; set; }
	}

	public class EdictDeclaration
	{
		[JsonProperty ("name")]
		public string Name { get; set; }

		[JsonProperty ("cost")]
		public Yield[] Cost { get; set; }

		[JsonProperty ("provides")]
		public Yield[] Provides { get; set; }

		[JsonProperty ("cooldown")]
		public int Cooldown { get; set; }

		[JsonProperty ("required_technology", NullValueHandling = NullValueHandling.Ignore)]
		public string RequireTechnology { get; set; }

		[JsonProperty ("required_building", NullValueHandling = NullValueHandling.Ignore)]
		public string RequireBuilding { get; set; }
	}

	public class EdictsDeclarations
	{
		[JsonProperty ("edicts")]
		public List<EdictDeclaration> Edicts { get; set; }

	}
}
