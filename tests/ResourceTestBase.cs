using System;
using System.Collections.Generic;
using System.Linq;

using Xunit;

using IncrementalSociety.Population;
using IncrementalSociety.Model;
using IncrementalSociety.Json;
using IncrementalSociety.Utilities;

namespace IncrementalSociety.Tests
{
	public abstract class ResourceTestBase
	{
		static JsonLoader CreateJsonLoader (string extraBuildingJSON = "", string extraGameJSON = "", string extraRegionJSON ="", string extraFeatureJSON = "", string extraResourceJSON = "", string extraResearchJSON = "", string extraEdictsJSON = "")
		{
			return new JsonLoader(BuildingJSON.Replace("%TEST_SPECIFIC%", extraBuildingJSON),
								GameJSON.Replace("%TEST_SPECIFIC%", extraGameJSON),
								AreaJSON.Replace("%AREA_SPECIFIC%", extraRegionJSON).Replace("%FEATURE_SPECIFIC%", extraFeatureJSON),
								ResourceJSON.Replace("%TEST_SPECIFIC%", extraResourceJSON),
								ResearchJSON.Replace("%TEST_SPECIFIC%", extraResearchJSON),
								EdictsJSON.Replace("%TEST_SPECIFIC%", extraEdictsJSON)); ;
		}

		protected void ConfigureCustomJsonPayload (string extraBuildingJSON = "", string extraGameJSON = "", string extraRegionJSON = "", string extraFeatureJSON = "", string extraResourceJSON = "", string extraResearchJSON = "", string extraEdictsJSON = "")
		{
			Loader = new Lazy<JsonLoader> (CreateJsonLoader (extraBuildingJSON, extraGameJSON, extraRegionJSON, extraFeatureJSON, extraResourceJSON, extraResearchJSON, extraEdictsJSON));
		}

		Lazy<JsonLoader> Loader = new Lazy<JsonLoader> (() => CreateJsonLoader ());

		Lazy<ResourceConfig> LazyConfig => new Lazy<ResourceConfig> (() => new ResourceConfig (Loader.Value.Resources.Resources.Select (x => x.Name)));
		protected ResourceConfig Config => LazyConfig.Value;

		Lazy<EdictCooldownConfig> LazyEdictConfig => new Lazy<EdictCooldownConfig> (() => new EdictCooldownConfig (Loader.Value.Edicts.Edicts.Select (x => x.Name)));
		protected EdictCooldownConfig EdictConfig => LazyEdictConfig.Value;

		protected Resources.Builder CreateBuilder (string resource, double amount)
		{
			var builder = Config.CreateBuilder ();
			builder[resource] = amount;
			return builder;
		}

		protected Resources Create (string resource, double amount) => CreateBuilder (resource, amount).ToResources ();

		const string ResourceJSON = @"{ ""resources"": [
			{
				""name"": ""Food""
			},
			{
				""name"": ""Water""
			},
			{
				""name"": ""Charcoal""
			},
			{
				""name"": ""Wood""
			},
			{
				""name"": ""Pottery""
			}
			%TEST_SPECIFIC%
		]
		}";

		const string BuildingJSON = @"{
		""buildings"": [
			{
				""name"": ""Gathering Camp"",
				""valid_areas"": [""Plains""],
				""yield"": [
					{ ""name"": ""Food"", ""amount"" : 2 },
					{ ""name"": ""Water"", ""amount"" : 2 }
				],
				""cost"": [
					{ ""name"": ""Wood"", ""amount"" : 10 },
				],
				""storage"": [
					{ ""name"": ""Food"", ""amount"" : 500 },
					{ ""name"": ""Water"", ""amount"" : 400 },
					{ ""name"": ""Wood"", ""amount"" : 50 },
				],
				""housing_capacity"": [
					{ ""capacity"": 200 }
				]
			},
			{
				""name"": ""Workshop"",
				""valid_areas"": [""Plains""],
				""conversion_yield"": [
					{
						""name"": ""Conversion"",
						""cost"": [
							{ ""name"": ""Wood"", ""amount"" : 1 }
						],
						""provides"": [
							{ ""name"": ""Charcoal"", ""amount"" : 0.5 }
						]
					}
				],
				""cost"": [
					{ ""name"": ""Wood"", ""amount"" : 10 }
				],
				""storage"": [
					{ ""name"": ""Charcoal"", ""amount"" : 50 },
				],
				""housing_capacity"": [
					{ ""capacity"": 50 }
				]
			},
			{
				""name"": ""Smoker"",
				""valid_areas"": [""Plains""],
				""conversion_yield"": [
					{
						""name"": ""OtherConversion"",
						""cost"": [
							{ ""name"": ""Charcoal"", ""amount"" : 1 }
						],
						""provides"": [
							{ ""name"": ""Food"", ""amount"" : 0.5 }
						]
					}
				],
				""storage"": [
					{ ""name"": ""Food"", ""amount"" : 500 },
				],
			},
			{
				""name"": ""Watering Hole"",
				""valid_areas"": [""Plains""],
				""prevent_destory"": true,
				""yield"": [
					{ ""name"": ""Water"", ""amount"" : 1.7 }
				]
			},
			{
				""name"": ""Housing"",
				""valid_areas"": [""Plains""],
				""does_not_require_job"": true,
				""housing_capacity"": [
					{ ""capacity"": 200 },
					{ ""required_technology"": ""Expansion"", ""capacity"": 200 }
				]
			},
			{
				""name"": ""Potter"",
				""valid_areas"": [""Plains""],
				""yield"": [
					{ ""name"": ""Pottery"", ""amount"" : .15 }
				]
			}
			%TEST_SPECIFIC%
		]
		}";

		const string AreaJSON = @"{
			""areas"": [
				{
					""name"": ""Plains""
				},
				{
					""name"": ""Mountains""
				}
				%AREA_SPECIFIC%
			],
			""features"": [
				%FEATURE_SPECIFIC%
			]
		}";

		const string GameJSON = @"{
			""population_needs"": [
				{ ""name"": ""Water"", ""amount"" : .01 },
				{ ""required_technology"": ""Bronze Age"", ""name"": ""Food"", ""amount"" : .01 },
			],
			""luxary_population_needs"": [
				{
					""name"": ""Pottery"", ""amount"": 0.01
				}
			],
			""region_capacity"": [
				{ ""capacity"": 3 },
				{  ""required_technology"": ""Expansion"", ""capacity"": 1 }
			],
			""min_population"" :  100,
			""happiness_gain_per_luxary_full"": 0.1,
			""happiness_loss_per_luxary_missing"": 0.2,
			""happiness_loss_pop_starting"": 10,
			""happiness_loss_per_extra_pop"": 0.1,
			""health_loss_pop_starting"": 10,
			""health_loss_per_extra_pop"": 0.1
			%TEST_SPECIFIC%
		}";

		const string ResearchJSON = @"{
			""research"" : [
				{
					""name"": ""FreeTech""
				},
				{
					""name"": ""TechWithCost"",
					""cost"": [ { ""name"": ""Food"", ""amount"" : 100 } ]
				},
				{
					""name"": ""TechWithDependency"",
					""Dependencies"": [ ""FreeTech"" ]
				}
				%TEST_SPECIFIC%
			]
		}";

		const string EdictsJSON = @"{
			""edicts"" : [
				%TEST_SPECIFIC%
			]
		}";

		protected GameState CreateGameState (int camps = 0, int workshops = 0, int smokers = 0, int holes = 0)
		{
			var buildings = new List<string> ();
			for (int i = 0 ; i < camps ; ++i)
				buildings.Add ("Gathering Camp");
			for (int i = 0 ; i < workshops ; ++i)
				buildings.Add ("Workshop");
			for (int i = 0 ; i < smokers ; ++i)
				buildings.Add ("Smoker");
			for (int i = 0 ; i < holes ; ++i)
				buildings.Add ("Watering Hole");
			return CreateGameState ((new Area ("Plains", buildings: buildings)).Yield ());
		}

		protected GameState CreateGameState (IEnumerable<Area> areas)
		{
			var resourceEngine = CreateResourceEngine ();
			var edictEngine = CreateEdictsEngine ();
			var region = new Region ("TestLand", areas);
			return new GameState (1, "Stone", region.Yield(), resourceEngine.ResourceConfig.Create (), 150, 200, edictEngine.EdictConfig.Create ());
		}

		protected PopUnits CreatePopUnits () => new PopUnits (Loader.Value.Game.MinPopulation);
		protected ResourceEngine CreateResourceEngine () => new ResourceEngine (Loader.Value);
		protected BuildingEngine CreateBuildingEngine () => new BuildingEngine (CreateResourceEngine (), CreatePopEngine ());
		protected ResearchEngine CreateResearchEngine () => new ResearchEngine (CreateResourceEngine (), Loader.Value);
		protected EdictsEngine CreateEdictsEngine () => new EdictsEngine (CreateResourceEngine (), Loader.Value);
		protected PopulationResources CreatePopulationResources () => new PopulationResources (CreateResourceEngine(), CreatePopulationBuildingInfo (), Loader.Value);
		protected PopulationCapacity CreatePopulationCapacity () => new PopulationCapacity (CreateResourceEngine(), CreatePopulationResources (), CreatePopulationBuildingInfo (), CreatePopUnits ());
		protected PopulationEngine CreatePopEngine () => new PopulationEngine (CreateResourceEngine(), CreatePopulationCapacity (), CreatePopulationResources (), Loader.Value);
		protected PopulationBuildingInfo CreatePopulationBuildingInfo () => new PopulationBuildingInfo (CreateResourceEngine(), CreatePopUnits ());
		protected PopulationGrowthCurve CreatePopulationGrowthCurve () => new PopulationGrowthCurve (CreatePopulationCapacity (), Loader.Value.Game.MinPopulation);
		protected PopulationNeeds CreatePopulationNeeds () => new PopulationNeeds (CreateResourceEngine (), Loader.Value, CreatePopUnits (), CreatePopulationResources ());
	}
}