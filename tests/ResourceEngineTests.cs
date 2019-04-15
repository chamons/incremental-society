using System.Collections.Immutable;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;
using Xunit;

namespace IncrementalSociety.Tests
{
	public class ResourceEngineTests
	{
		const string ResourceJSON = @"{    ""resources"": [
        {
            ""name"": ""Food"",
        },
        {
            ""name"": ""Water"",
        }
		]
}";

		const string BuildingJSON = @"{	""buildings"": [
		{
			""name"": ""Gathering Camp"",
			""valid_regions"": [""Plains""],
			""yield"": [
				{ ""Name"": ""Food"", ""Amount"" : 2 },
				{ ""Name"": ""Water"", ""Amount"" : 2 }
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


		static ResourceEngine CreateEngine ()
		{
			var resources = new JsonLoader ("", BuildingJSON, "", RegionJSON, ResourceJSON);
			ResourceEngine engine = new ResourceEngine (resources);
			return engine;
		}

		// TODO - This should use test specific  resources!
		static GameState CreateGameState ()
		{
			var area = new Area (AreaType.Plains, "Gathering Camp".Yield ());
			var region = new Region ("TestLand", area.Yield ());
			return new GameState (Age.Stone, region.Yield(), new System.Collections.Generic.Dictionary<string, double> ());
		}

		[Fact]
		public void AdditionalResourceNextTick ()
		{
			ResourceEngine engine = CreateEngine ();
			GameState state = CreateGameState ();
			var resources = engine.CalculateAdditionalNextTick (state);
			Assert.True (resources["Food"] > 0.0);
			Assert.True (resources["Water"] > 0.0);
		}

		[Fact]
		public void BuildingResources ()
		{
			ResourceEngine engine = CreateEngine ();
			var campResources = engine.GetBuildingResources ("Gathering Camp");
			Assert.True (campResources["Food"] > 0.0);
			Assert.True (campResources["Water"] > 0.0);
		}

		[Fact]
		public void AddTwoResourcesDifferentItems ()
		{
			var result = Immutable.CreateBuilderDictionary ("Food", 1.0);
			ResourceEngine.AddResources (result, Immutable.CreateDictionary ("Water", 1.0));
			Assert.Equal (1, result["Food"]);
			Assert.Equal (1, result["Water"]);
		}

		[Fact]
		public void AddTwoResourcesWithSameItems ()
		{
			var result = Immutable.CreateBuilderDictionary ("Food", 1.0);
			ResourceEngine.AddResources (result, Immutable.CreateDictionary ("Food", 1.0));
			Assert.Equal (2, result["Food"]);
		}

		[Fact]
		public void AddTwoResourceOneEmpty ()
		{
			var result = Immutable.CreateBuilderDictionary ("Food", 1.0);
			ResourceEngine.AddResources (result, ImmutableDictionary<string, double>.Empty);
			Assert.Equal (1, result["Food"]);
		}
	}
}
