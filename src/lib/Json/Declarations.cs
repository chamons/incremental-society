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
		public string Age { get; set; }
		public bool Image_has_age_prefix { get; set; }
	}

	public class ResourceDeclarations
	{
		public List<ResourceDeclaration> Resources { get; set; }
	}

	public class RegionDeclaration
	{
		public string Name { get; set; }
		public List<string> Resources { get; set; }
		public bool? Can_spawn { get; set; }
	}

	public class RegionDeclarations
	{
		public List<RegionDeclaration> Regions { get; set; }
	}

	public class PopulationNeeds
	{
		public string Age { get; set; }
		public string Resource { get; set; }
		public double Amount { get; set; }
		public string Missing_effect { get; set; }
		public double Missing_power { get; set; }
	}

	public class GameDeclarations
	{
		public List<string> Ages { get; set; }
		public List<PopulationNeeds> Population_needs { get; set; }
	}

	public partial class BuildingDeclarations
	{
		[JsonProperty ("buildings")]
		public Building[] Buildings { get; set; }

		[JsonProperty ("settlements")]
		public Settlement[] Settlements { get; set; }
	}

	public partial class Building
	{
		[JsonProperty ("name")]
		public string Name { get; set; }

		[JsonProperty ("introduced_age")]
		public string IntroducedAge { get; set; }

		[JsonProperty ("valid_regions")]
		public string ValidRegions { get; set; }

		[JsonProperty ("yield", NullValueHandling = NullValueHandling.Ignore)]
		public Yield[] Yield { get; set; }

		[JsonProperty ("conversion_yield", NullValueHandling = NullValueHandling.Ignore)]
		public ConversionYield[] ConversionYield { get; set; }

		[JsonProperty ("RequiredResource ", NullValueHandling = NullValueHandling.Ignore)]
		public string RequiredResource { get; set; }
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

	public partial class Settlement
	{
		[JsonProperty ("name")]
		public string Name { get; set; }

		[JsonProperty ("introduced_age")]
		public string IntroducedAge { get; set; }

		[JsonProperty ("valid_regions")]
		public string ValidRegions { get; set; }

		[JsonProperty ("yield")]
		public Yield[] Yield { get; set; }
	}

	public class GameAction
	{
		public string Name { get; set; }
		public string Age { get; set; }
	}

	public class ActionDeclarations
	{
		public List<GameAction> Actions { get; set; }
	}
}
