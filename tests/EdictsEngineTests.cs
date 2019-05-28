using System;
using System.Collections.Generic;
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
			Assert.False (state.Edicts["Edict"] == 2);
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
			// Multiple edicts, some which require buildings we do/don't have. Same with tech
			// One of which is on cooldown
		}
	}
}
