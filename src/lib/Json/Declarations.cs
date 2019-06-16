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

	public class AreaDeclaration
	{
		[JsonProperty ("name")]
		public string Name { get; set; }
		[JsonProperty ("resources")]
		public List<string> Resources { get; set; }
		[JsonProperty ("can_spawn")]
		public bool? CanSpawn { get; set; }
		[JsonProperty ("bonus_yield")]
		public Yield[] BonusYield { get; set; }
	}

	public class FeatureDeclaration
	{
		[JsonProperty("name")]
		public string Name { get; set; }

		[JsonProperty("bonus_yield")]
		public Yield[] BonusYield { get; set; }
	}

	public class AreaProbability
	{
		[JsonProperty("name")]
		public string Name { get; set; }

		[JsonProperty("chance")]
		public double Chance { get; set; }
	}

	public class ClimateDeclaration
	{
		[JsonProperty("name")]
		public string Name { get; set; }

		[JsonProperty("area_chances")]
		public AreaProbability[] AreaChances { get; set; }

		[JsonProperty ("feature_chance")]
		public double FeatureChance { get; set; }
	}

	public class AreaDeclarations
	{
		[JsonProperty ("areas")]
		public List<AreaDeclaration> Areas { get; set; }

		[JsonProperty ("features")]
		public List<FeatureDeclaration> Features { get; set; }

		[JsonProperty ("climates")]
		public List<ClimateDeclaration> Climates { get; set; }
	}

	public class GameDeclarations
	{
		[JsonProperty ("population_needs")]
		public Yield[] PopulationNeeds { get; set; }

		[JsonProperty("luxury_population_needs")]
		public Yield[] LuxuryPopulationNeeds { get; set; }

		[JsonProperty ("region_capacity")]
		public CapacityDeclaration [] RegionCapacityDeclarations { get; set; }

		[JsonProperty ("min_population")]
		public int MinPopulation { get; set; }

		[JsonProperty ("happiness_gain_per_luxury_full")]
		public double HappinessGainPerFullLuxury { get; set; }

		[JsonProperty ("happiness_loss_per_luxury_missing")]
		public double HappinessLossPerLuxuryMissing { get; set; }

		[JsonProperty ("happiness_loss_pop_starting")]
		public double HappinessLossStaring { get; set; }

		[JsonProperty ("happiness_loss_per_extra_pop")]
		public double HappinessLossPerExtraPop { get; set; }

		[JsonProperty ("health_loss_pop_starting")]
		public double HealthLossStaring { get; set; }

		[JsonProperty ("health_loss_per_extra_pop")]
		public double HealthLossPerExtraPop { get; set; }

		[JsonProperty ("base_pop_growth_rate")]
		public double BasePopGrowthRate { get; set; }

		[JsonProperty ("min_growth")]
		public double MinGrowth { get; set; }

		[JsonProperty ("base_immigration_rate")]
		public double BaseImmigrationRate { get; set; }

		[JsonProperty ("base_emigration_rate")]
		public double BaseEmmigrationRate { get; set; }

		[JsonProperty ("housing_emigration_rate_per_missing")]
		public double HousingEmmigrationRatePerMissing { get; set; }

		[JsonProperty ("housing_emigration_rate_base")]
		public double HousingEmmigrationRateBase { get; set; }

		[JsonProperty ("base_death_rate")]
		public double BaseDeathRate { get; set; }
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

		[JsonProperty ("valid_areas")]
		public string[] ValidAreas { get; set; }

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

		[JsonProperty ("isNotStandalone")]
		public bool IsNotStandalone { get; set; }

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
