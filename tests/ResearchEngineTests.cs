using System;
using IncrementalSociety.Model;
using Xunit;

namespace IncrementalSociety.Tests
{
	public class ResearchEngineTests : ResourceTestBase
	{
		[Fact]
		public void ResearchAppliesToState ()
		{
			ResearchEngine engine = CreateResearchEngine ();
			GameState state = CreateGameState ();

			Assert.True (engine.CanResearch (state, "FreeTech"));
			state = engine.Research (state, "FreeTech");

			Assert.Single (state.ResearchUnlocks);
			Assert.Contains (state.ResearchUnlocks, x => x == "FreeTech");
		}

		[Fact]
		public void ResearchRequiresResources ()
		{
			ResearchEngine engine = CreateResearchEngine ();
			GameState state = CreateGameState ();

			Assert.False (engine.CanResearch (state, "TechWithCost"));
			Assert.Throws<InvalidOperationException> (() => engine.Research (state, "TechWithCost"));

			state = state.WithResources (Create ("Food", 100));
			Assert.True (engine.CanResearch (state, "TechWithCost"));
			state = engine.Research (state, "TechWithCost");

			Assert.Single (state.ResearchUnlocks);
			Assert.Contains (state.ResearchUnlocks, x => x == "TechWithCost");
			Assert.Equal (0, state.Resources["Food"]);
		}

		[Fact]
		public void ResearchHonorsDependencies ()
		{
			ResearchEngine engine = CreateResearchEngine ();
			GameState state = CreateGameState ();

			Assert.False (engine.CanResearch (state, "TechWithDependency"));
			Assert.Throws<InvalidOperationException> (() => engine.Research (state, "TechWithDependency"));

			state = engine.Research (state, "FreeTech");

			Assert.True (engine.CanResearch (state, "TechWithDependency"));
			state = engine.Research (state, "TechWithDependency");

			Assert.Equal (2, state.ResearchUnlocks.Count);
			Assert.Contains (state.ResearchUnlocks, x => x == "TechWithDependency");
		}

		[Fact]
		public void CanNotDoubleResearchItem ()
		{
			ResearchEngine engine = CreateResearchEngine ();
			GameState state = CreateGameState ();

			Assert.True (engine.CanResearch (state, "FreeTech"));
			state = engine.Research (state, "FreeTech");

			Assert.False (engine.CanResearch (state, "FreeTech"));
			Assert.Throws<InvalidOperationException> (() => engine.Research (state, "FreeTech"));

			Assert.Single (state.ResearchUnlocks);
			Assert.Contains (state.ResearchUnlocks, x => x == "FreeTech");
		}

		[Fact]
		public void CanNotResearchNonStandAloneItem ()
		{
			const string extraResearchJson = @",
			{
				""name"": ""DoesNotStandAlone"",
				""isNotStandalone"": true
			}";
			ConfigureCustomJsonPayload (extraResearchJSON: extraResearchJson);

			ResearchEngine engine = CreateResearchEngine ();
			GameState state = CreateGameState ();

			Assert.False (engine.CanResearch (state, "DoesNotStandAlone"));
			Assert.Throws<InvalidOperationException> (() => engine.Research (state, "DoesNotStandAlone"));
		}

		[Fact]
		public void ResearchOptions ()
		{
			ResearchEngine engine = CreateResearchEngine ();
			GameState state = CreateGameState ();

			var availableResearch = engine.GetCurrentResearchOptions (state);
			Assert.Equal (2, availableResearch.Count);
			Assert.Contains (availableResearch, x => x.Name == "FreeTech");
			Assert.Contains (availableResearch, x => x.Name == "TechWithCost");

			state = engine.Research (state, "FreeTech");

			availableResearch = engine.GetCurrentResearchOptions (state);
			Assert.Equal (2, availableResearch.Count);
			Assert.Contains (availableResearch, x => x.Name == "TechWithDependency");
			Assert.Contains (availableResearch, x => x.Name == "TechWithCost");
		}

		[Fact]
		public void SpecializationThrowsIfResearchWithoutSelection ()
		{
			const string extraResearchJson = @",
			{
				""name"": ""Special"",
				""specializations"": [ ""First"", ""Second"", ""Third"" ]
			}";
			ConfigureCustomJsonPayload (extraResearchJSON: extraResearchJson);

			ResearchEngine engine = CreateResearchEngine ();
			GameState state = CreateGameState ();

			Assert.Throws<InvalidOperationException> (() => engine.CanResearch (state, "Special"));
			Assert.Throws<InvalidOperationException> (() => engine.Research (state, "Special"));
		}

		[Fact]
		public void NonSpecializationThrowsIfResearchWithSelection ()
		{
			ResearchEngine engine = CreateResearchEngine ();
			GameState state = CreateGameState ();

			Assert.Throws<InvalidOperationException> (() => engine.CanResearch (state, "FreeTech", 1));
			Assert.Throws<InvalidOperationException> (() => engine.Research (state, "FreeTech", 1));
		}

		[Fact]
		public void SpecializationGrantsSepecializationIfResearched ()
		{
			const string extraResearchJson = @",
			{
				""name"": ""Special"",
				""specializations"": [ ""First"", ""Second"", ""Third"" ]
			}";
			ConfigureCustomJsonPayload (extraResearchJSON: extraResearchJson);

			ResearchEngine engine = CreateResearchEngine ();
			GameState state = CreateGameState ();

			Assert.True (engine.CanResearch (state, "Special", 1));
			state = engine.Research (state, "Special", 1);
			Assert.Equal (2, state.ResearchUnlocks.Count);
			Assert.Contains (state.ResearchUnlocks, x => x == "Special");
			Assert.Contains (state.ResearchUnlocks, x => x == "Second");
		}

		[Fact]
		public void SpecializationCanNotBeSelectedTwice ()
		{
			const string extraResearchJson = @",
			{
				""name"": ""Special"",
				""specializations"": [ ""First"", ""Second"", ""Third"" ]
			}";
			ConfigureCustomJsonPayload (extraResearchJSON: extraResearchJson);

			ResearchEngine engine = CreateResearchEngine ();
			GameState state = CreateGameState ();

			Assert.True (engine.CanResearch (state, "Special", 1));
			state = engine.Research (state, "Special", 1);

			Assert.False (engine.CanResearch (state, "Special", 0));
			Assert.Throws<InvalidOperationException> (() => engine.Research (state, "Special", 0));
			Assert.False (engine.CanResearch (state, "Special", 10));
			Assert.Throws<InvalidOperationException> (() => engine.Research (state, "Special", 10));
		}

		[Fact]
		public void SecondSpecializationCanSelectOnlyOtherSpecializations ()
		{
			const string extraResearchJson = @",
			{
				""name"": ""Special"",
				""specializations"": [ ""First"", ""Second"", ""Third"" ]
			},
			{
				""name"": ""Special 2"",
				""specializations"": [ ""First"", ""Second"", ""Third"" ]
			}";
			ConfigureCustomJsonPayload (extraResearchJSON: extraResearchJson);

			ResearchEngine engine = CreateResearchEngine ();
			GameState state = CreateGameState ();

			Assert.True (engine.CanResearch (state, "Special", 1));
			state = engine.Research (state, "Special", 1);

			Assert.False (engine.CanResearch (state, "Special", 1));
			Assert.Throws<InvalidOperationException> (() => engine.Research (state, "Special", 1));

			Assert.True (engine.CanResearch (state, "Special 2", 0));
			state = engine.Research (state, "Special 2", 0);

			Assert.Equal (4, state.ResearchUnlocks.Count);
			Assert.Contains (state.ResearchUnlocks, x => x == "Special");
			Assert.Contains (state.ResearchUnlocks, x => x == "Special 2");
			Assert.Contains (state.ResearchUnlocks, x => x == "First");
			Assert.Contains (state.ResearchUnlocks, x => x == "Second");
		}
	}
}
