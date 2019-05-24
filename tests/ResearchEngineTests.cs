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
			ResearchEngine engine = Factories.CreateResearchEngine ();
			GameState state = Factories.CreateGameState ();

			Assert.True (engine.CanResearch (state, "FreeTech"));
			state = engine.Research (state, "FreeTech");

			Assert.Single (state.ResearchUnlocks);
			Assert.Contains (state.ResearchUnlocks, x => x == "FreeTech");
		}

		[Fact]
		public void ResearchRequiresResources ()
		{
			ResearchEngine engine = Factories.CreateResearchEngine ();
			GameState state = Factories.CreateGameState ();

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
			ResearchEngine engine = Factories.CreateResearchEngine ();
			GameState state = Factories.CreateGameState ();

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
			ResearchEngine engine = Factories.CreateResearchEngine ();
			GameState state = Factories.CreateGameState ();

			Assert.True (engine.CanResearch (state, "FreeTech"));
			state = engine.Research (state, "FreeTech");

			Assert.False (engine.CanResearch (state, "FreeTech"));
			Assert.Throws<InvalidOperationException> (() => engine.Research (state, "FreeTech"));

			Assert.Single (state.ResearchUnlocks);
			Assert.Contains (state.ResearchUnlocks, x => x == "FreeTech");
		}

		[Fact]
		public void ResearchOptions ()
		{
			ResearchEngine engine = Factories.CreateResearchEngine ();
			GameState state = Factories.CreateGameState ();

			var availableResearch = engine.GetCurrentResearchOptions (state);
			Assert.Equal (2, availableResearch.Count);
			Assert.Contains (availableResearch, x => x.Name == "FreeTech");
			Assert.Contains (availableResearch, x => x.Name == "TechWithCost");

			state = engine.Research (state, "FreeTech");

			availableResearch = engine.GetCurrentResearchOptions (state);
			Assert.Equal (3, availableResearch.Count);
			Assert.Contains (availableResearch, x => x.Name == "FreeTech" && x.IsResearched);
			Assert.Contains (availableResearch, x => x.Name == "TechWithDependency" && !x.IsResearched);
			Assert.Contains (availableResearch, x => x.Name == "TechWithCost" && !x.IsResearched);
		}
	}
}
