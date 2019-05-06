using System.Collections.Immutable;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;
using Xunit;

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
				]
			},
			{
				""name"": ""Workshop"",
				""valid_regions"": [""Plains""],
				""conversion_yield"": [
					{
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
				]

			},
			{
				""name"": ""Mine"",
				""valid_regions"": [""Mountains""]
			}
		],
		""settlements"": [
			{
				""name"": ""Test Settlement"",
				""valid_regions"": [""Plains""],
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

		const string GameJSON = @"{ ""region_capacity"" :  2 }";

		public static ResourceEngine CreateResourceEngine ()
		{
			var resources = new JsonLoader ("", BuildingJSON, GameJSON, RegionJSON, ResourceJSON);
			ResourceEngine engine = new ResourceEngine (resources);
			return engine;
		}

		public static GameState CreateGameStateWithOneCamp ()
		{
			return CreateGameState (new Area (AreaType.Plains, "Gathering Camp".Yield ()));
		}

		public static GameState CreateGameStateFullOfCamps ()
		{
			return CreateGameState (new Area (AreaType.Plains, new string [] { "Gathering Camp", "Gathering Camp"}));
		}

		static GameState CreateGameState (Area area)
		{
			var region = new Region ("TestLand", area.Yield ());
			return new GameState (Age.Stone, region.Yield(), new System.Collections.Generic.Dictionary<string, double> ());
		}


		public static BuildingEngine CreateBuildingEngine ()
		{
			return new BuildingEngine (CreateResourceEngine ());
		}
	}
}
