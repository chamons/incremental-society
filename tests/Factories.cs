using System.Collections.Generic;

using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

namespace IncrementalSociety.Tests
{
	public class Factories
	{
		const string ResourceJSON = @"{ ""resources"": [
			{
				""name"": ""Food"",
			},
			{
				""name"": ""Water"",
			},
			{
				""name"": ""Charcoal"",
			},
			{
				""name"": ""Wood"",
			}
		]
}";

		const string BuildingJSON = @"{	
		""buildings"": [
			{
				""name"": ""Gathering Camp"",
				""valid_regions"": [""Plains""],
				""yield"": [
					{ ""Name"": ""Food"", ""Amount"" : 2 },
					{ ""Name"": ""Water"", ""Amount"" : 2 }
				],
				""cost"": [
					{ ""Name"": ""Wood"", ""Amount"" : 10 },
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
							{ ""Name"": ""Wood"", ""Amount"" : 1 }
						],
						""provides"": [ 
							{ ""Name"": ""Charcoal"", ""Amount"" : 0.5 }
						]
					}
				],
				""cost"": [
					{ ""Name"": ""Wood"", ""Amount"" : 10 }
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
							{ ""Name"": ""Charcoal"", ""Amount"" : 1 }
						],
						""provides"": [ 
							{ ""Name"": ""Food"", ""Amount"" : 0.5 }
						]
					}
				]
			},
			{
				""name"": ""Mine"",
				""valid_regions"": [""Mountains""]
			}
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
	}
";

		const string GameJSON = @"{
			""population_needs"": [	{
				""Name"": ""Water"", ""Amount"" : 1,
			}],
			""region_capacity"" :  2,
			""min_population"" :  100
		}";

		static JsonLoader CreateJsonLoader ()
		{
			return new JsonLoader ("", BuildingJSON, GameJSON, RegionJSON, ResourceJSON);
		}

		public static GameState CreateGameState (int camps = 0, int workshops = 0, int smokers = 0)
		{
			var buildings = new List<string> ();
			for (int i = 0 ; i < camps ; ++i)
				buildings.Add ("Gathering Camp");
			for (int i = 0 ; i < workshops ; ++i)
				buildings.Add ("Workshop");
			for (int i = 0 ; i < smokers ; ++i)
				buildings.Add ("Smoker");
			return CreateGameState (new Area (AreaType.Plains, buildings));
		}

		static GameState CreateGameState (Area area)
		{
			var region = new Region ("TestLand", area.Yield ());
			return new GameState (Age.Stone, region.Yield(), new Dictionary<string, double> (), 150, 200);
		}

		public static ResourceEngine CreateResourceEngine ()
		{
			return new ResourceEngine (CreateJsonLoader ());
		}

		public static BuildingEngine CreateBuildingEngine ()
		{
			return new BuildingEngine (CreateResourceEngine ());
		}

		public static PopulationEngine CreatePopEngine ()
		{
			return new PopulationEngine (CreateResourceEngine(), CreateJsonLoader ());
		}
	}
}
