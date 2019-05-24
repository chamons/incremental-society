using System;
using System.Collections.Generic;
using System.Linq;

using Xunit;

using IncrementalSociety.Model;
using IncrementalSociety.Json;
using IncrementalSociety.Utilities;

namespace IncrementalSociety.Tests
{
	public abstract class ResourceTestBase
	{
		protected ResourceConfig Config;

		protected ResourceTestBase ()
		{
			JsonLoader loader = new JsonLoader ("", "", "", ResourceJSON, "", validate: false);
			Config = new ResourceConfig (loader.Resources.Resources.Select (x => x.Name));
		}

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
			}
		]
		}";

		const string BuildingJSON = @"{
		""buildings"": [
			{
				""name"": ""Gathering Camp"",
				""valid_regions"": [""Plains""],
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
				""housing_capacity"": 200
			},
			{
				""name"": ""Workshop"",
				""valid_regions"": [""Plains""],
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
				""housing_capacity"": 50
			},
			{
				""name"": ""Smoker"",
				""valid_regions"": [""Plains""],
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
				""valid_regions"": [""Plains""],
				""prevent_destory"": true,
				""yield"": [
					{ ""name"": ""Water"", ""amount"" : 1.7 }
				]
			},
			{
				""name"": ""Housing"",
				""valid_regions"": [""Plains""],
				""housing_capacity"": 200
			},
			{
				""name"": ""Impossible"",
				""valid_regions"": [""Plains""],
				""prevent_build"" : true
			},
			{
				""name"": ""Any"",
				""valid_regions"": [""Any""]
			},
			{
				""name"": ""Mine"",
				""valid_regions"": [""Mountains""]
			},
			{
				""name"": ""NoJob"",
				""valid_regions"": [""Any""],
				""does_not_require_job"": true
			},
		]
		}";

		const string RegionJSON = @"{
			""regions"": [
				{
					""name"": ""Plains""
				},
				{
					""name"": ""Mountains""
				}
			]
		}";

		const string GameJSON = @"{
			""population_needs"": [	{
				""name"": ""Water"", ""amount"" : .01,
			}],
			""region_capacity"" :  3,
			""min_population"" :  100
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
				},
			]
		}";

		protected JsonLoader CreateJsonLoader ()
		{
			return new JsonLoader (BuildingJSON, GameJSON, RegionJSON, ResourceJSON, ResearchJSON);
		}

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
			return CreateGameState (new Area (AreaType.Plains, buildings));
		}

		protected GameState CreateGameState (Area area)
		{
			var resourceEngine = CreateResourceEngine ();
			var region = new Region ("TestLand", area.Yield ());
			return new GameState (1, Age.Stone, region.Yield(), resourceEngine.ResourceConfig.Create (), 150, 200);
		}

		protected ResourceEngine CreateResourceEngine ()
		{
			return new ResourceEngine (CreateJsonLoader ());
		}

		protected BuildingEngine CreateBuildingEngine ()
		{
			return new BuildingEngine (CreateResourceEngine (), CreatePopEngine ());
		}

		protected PopulationEngine CreatePopEngine ()
		{
			return new PopulationEngine (CreateResourceEngine(), CreateJsonLoader ());
		}

		protected ResearchEngine CreateResearchEngine ()
		{
			return new ResearchEngine (CreateResourceEngine (), CreateJsonLoader ());
		}
	}
}