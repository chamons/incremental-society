using System;
using System.Collections.Generic;
using System.Text;
using Newtonsoft.Json;

namespace IncrementalSociety.Json
{
	public class ResourceDeclaration
	{
		public string Name { get; set; }
		public string Type { get; set; }
		[JsonProperty ("Image_has_age_prefix")]
		public bool ImageHasAgePrefix { get; set; }
	}

	public class ResourceDeclarations
	{
		public List<ResourceDeclaration> Resources { get; set; }
	}

	public class RegionDeclaration
	{
		public string Name { get; set; }
		public List<string> Resources { get; set; }
		[JsonProperty ("can_spawn")]
		public bool? CanSpawn { get; set; }
	}

	public class RegionDeclarations
	{
		public List<RegionDeclaration> Regions { get; set; }
	}

	public class PopulationNeeds
	{
		[JsonProperty ("resource")]
		public Yield[] Resource;

		[JsonProperty ("missing_power")]
		public double MissingPower { get; set; }
	}

	public class GameDeclarations
	{
		[JsonProperty ("population_needs")]
		public List<PopulationNeeds> PopulationNeeds { get; set; }

		[JsonProperty ("region_capacity")]
		public int RegionCapacity { get; set; }
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

		[JsonProperty ("valid_regions")]
		public string[] ValidRegions { get; set; }

		[JsonProperty ("yield", NullValueHandling = NullValueHandling.Ignore)]
		public Yield[] Yield { get; set; }

		[JsonProperty ("conversion_yield", NullValueHandling = NullValueHandling.Ignore)]
		public ConversionYield[] ConversionYield { get; set; }

		[JsonProperty ("RequiredResource ", NullValueHandling = NullValueHandling.Ignore)]
		public string RequiredResource { get; set; }
		
		[JsonProperty ("cost")]
		public Yield[] Cost { get; set; }

		[JsonProperty ("housing_capacity")]
		public int HousingCapacity { get; set; }
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
		[JsonProperty ("Name")]
		public string Name { get; set; }

		[JsonProperty ("Amount")]
		public double Amount { get; set; }
	}

	public class ActionDeclarations
	{
		public List<string> Actions { get; set; }
	}
}
