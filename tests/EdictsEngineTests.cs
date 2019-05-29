using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using Xunit;

namespace IncrementalSociety.Tests
{
	public class EdictsEngineTests : ResourceTestBase
	{
		[Fact]
		public void EdictsHaveCooldown ()
		{
			const string extraEdictsJSON = @"
				{
					""name"": ""Edict"",
					""cooldown"": 2
				}
			";

			ConfigureCustomJsonPayload (extraEdictsJSON: extraEdictsJSON);

			var engine = CreateEdictsEngine ();
			var state = CreateGameState ();
			Assert.True (engine.CanApplyEdict (state, "Edict"));
			state = engine.ApplyEdict (state, "Edict");

			Assert.False(engine.CanApplyEdict (state, "Edict"));
			Assert.True (state.Edicts["Edict"] == 2);
			Assert.Throws<InvalidOperationException> (() => engine.ApplyEdict (state, "Edict"));

			state = engine.ProcessTick (state);
			Assert.True (state.Edicts["Edict"] == 1);
			Assert.False (engine.CanApplyEdict (state, "Edict"));
			Assert.Throws<InvalidOperationException> (() => engine.ApplyEdict (state, "Edict"));

			state = engine.ProcessTick (state);
			Assert.True (state.Edicts["Edict"] == 0);
			Assert.True (engine.CanApplyEdict (state, "Edict"));
			state = engine.ApplyEdict (state, "Edict");
		}

		[Fact]
		public void EdictsHaveRequiredCostAndProvideResources ()
		{
			const string extraEdictsJSON = @"
				{
					""name"": ""Edict"",
					""cost"": [
						{ ""name"": ""Food"", ""amount"" : 10 }
					],
					""provides"": [
						{ ""name"": ""Wood"", ""amount"" : 2 }
					],
				}
			";

			ConfigureCustomJsonPayload (extraEdictsJSON: extraEdictsJSON);

			var engine = CreateEdictsEngine ();
			var state = CreateGameState ();
			Assert.False (engine.CanApplyEdict (state, "Edict"));
			state = state.WithResources (Create ("Food", 10));
			Assert.True (engine.CanApplyEdict (state, "Edict"));

			state = engine.ApplyEdict (state, "Edict");
			Assert.Equal (0, state.Resources["Food"]);
			Assert.Equal (2, state.Resources["Wood"]);
		}

		[Fact]
		public void EdictsCanHaveRequiredTechs ()
		{
			const string extraEdictsJSON = @"
				{
					""name"": ""Edict"",
					""required_technology"": ""Tech""
				}
			";

			ConfigureCustomJsonPayload (extraEdictsJSON: extraEdictsJSON);

			var engine = CreateEdictsEngine ();
			var state = CreateGameState ();
			Assert.False (engine.CanApplyEdict (state, "Edict"));
			state = state.WithResearchUnlocks (new string[] { "Tech" });
			Assert.True (engine.CanApplyEdict (state, "Edict"));
		}

		[Fact]
		public void EdictsCanHaveRequiredBuildings ()
		{
			const string extraEdictsJSON = @"
				{
					""name"": ""Edict"",
					""required_building"": ""Smoker""
				}
			";

			ConfigureCustomJsonPayload (extraEdictsJSON: extraEdictsJSON);

			var engine = CreateEdictsEngine ();
			var state = CreateGameState ();
			Assert.False (engine.CanApplyEdict (state, "Edict"));
			state = CreateGameState (smokers: 1);
			Assert.True (engine.CanApplyEdict (state, "Edict"));
		}

		[Fact]
		public void ListOfEdicts ()
		{
			const string extraEdictsJSON = @"
				{
					""name"": ""Edict""
				},
				{
					""name"": ""EdictWithCooldown"",
					""cooldown"": 2
				},
				{
					""name"": ""RequireTechEdict"",
					""required_technology"": ""Tech""
				},
				{
					""name"": ""RequireBuildingEdict"",
					""required_building"": ""Smoker""
				}
			";

			ConfigureCustomJsonPayload (extraEdictsJSON: extraEdictsJSON);

			var engine = CreateEdictsEngine ();
			var state = CreateGameState ();

			var edictList = engine.AvailableEdicts (state).ToList ();
			Assert.Equal (2, edictList.Count);
			Assert.Contains (edictList, x => x.Name == "Edict" && x.Cooldown == 0);
			Assert.Contains (edictList, x => x.Name == "EdictWithCooldown" && x.Cooldown == 0);

			state = engine.ApplyEdict (state, "EdictWithCooldown");
			edictList = engine.AvailableEdicts (state).ToList ();
			Assert.Equal (2, edictList.Count);
			Assert.Contains (edictList, x => x.Name == "Edict" && x.Cooldown == 0);
			Assert.Contains (edictList, x => x.Name == "EdictWithCooldown" && x.Cooldown == 2);

			state = state.WithResearchUnlocks (new string[] { "Tech" });
			edictList = engine.AvailableEdicts (state).ToList ();
			Assert.Equal (3, edictList.Count);
			Assert.Contains (edictList, x => x.Name == "RequireTechEdict");

			var buildingEngine = CreateBuildingEngine ();
			state = buildingEngine.Build (state, state.Regions[0].Name, 0, "Smoker");
			edictList = engine.AvailableEdicts (state).ToList ();
			Assert.Equal (4, edictList.Count);
			Assert.Contains (edictList, x => x.Name == "RequireBuildingEdict");
		}
	}
}
