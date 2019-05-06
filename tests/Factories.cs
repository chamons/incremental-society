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
			]
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
				}
			]
	}
";

		public static ResourceEngine CreateResourceEngine ()
		{
			var resources = new JsonLoader ("", BuildingJSON, "", RegionJSON, ResourceJSON);
			ResourceEngine engine = new ResourceEngine (resources);
			return engine;
		}

		public static GameState CreateGameStateWithOneCamp ()
		{
			var area = new Area (AreaType.Plains, "Gathering Camp".Yield ());
			var region = new Region ("TestLand", area.Yield ());
			return new GameState (Age.Stone, region.Yield(), new System.Collections.Generic.Dictionary<string, double> ());
		}

	}
}
