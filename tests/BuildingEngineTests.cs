using System;
using System.Collections.Immutable;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;
using Xunit;

namespace IncrementalSociety.Tests
{
	public class BuildingEngineTests : ResourceTestBase
	{
		[Fact]
		public void BuildValidBuilding ()
		{
			GameState state = CreateGameState (camps: 1);
			state = state.WithResources (Create ("Wood", 10.0));
			BuildingEngine engine = CreateBuildingEngine ();
			state = engine.Build (state, state.Regions[0].Name, 0, "Workshop");
			Assert.Equal (2, state.Regions[0].Areas[0].Buildings.Length);
			Assert.Equal (0, state.Resources["Wood"]);
		}

		[Fact]
		public void BuildBuildingWhereNoRoom ()
		{
			GameState state = CreateGameState (camps: 3);
			state = state.WithResources (Create ("Wood", 10.0));
			BuildingEngine engine = CreateBuildingEngine ();

			Assert.Throws<InvalidOperationException> (() => engine.Build (state, state.Regions[0].Name, 0, "Workshop"));
		}

		[Fact]
		public void BuildBuildingInvalidRegionType ()
		{
			const string extraBuildingJSON = @",
			{
				""name"": ""Mine"",
				""valid_areas"": [""Mountains""]
			}";
			ConfigureCustomJsonPayload (extraBuildingJSON: extraBuildingJSON);

			GameState state = CreateGameState (camps: 1);
			state = state.WithResources (Create ("Wood", 10.0));
			BuildingEngine engine = CreateBuildingEngine ();
			Assert.Throws<InvalidOperationException> (() => engine.Build (state, state.Regions[0].Name, 0, "Mine"));
		}

		[Fact]
		public void BuildBuildingValidAnywhere ()
		{
			const string extraBuildingJSON = @",
			{
				""name"": ""Any"",
				""valid_areas"": [""Any""]
			}";
			ConfigureCustomJsonPayload (extraBuildingJSON: extraBuildingJSON);


			GameState state = CreateGameState (camps: 1);
			state = state.WithResources (Create ("Wood", 10.0));
			BuildingEngine engine = CreateBuildingEngine ();
			engine.Build (state, state.Regions[0].Name, 0, "Any");
		}

		[Fact]
		public void BuildBuildingWithoutResourcs ()
		{
			GameState state = CreateGameState (camps: 1);
			BuildingEngine engine = CreateBuildingEngine ();
			Assert.Throws<InvalidOperationException> (() => engine.Build (state, state.Regions[0].Name, 0, "Workshop"));
		}

		[Fact]
		public void CannotBuildBuildingMarkedUnable ()
		{
			const string extraBuildingJSON = @",{
				""name"": ""Impossible"",
				""valid_areas"": [""Plains""],
				""prevent_build"" : true
			}";
			ConfigureCustomJsonPayload (extraBuildingJSON: extraBuildingJSON);

			GameState state = CreateGameState ();
			BuildingEngine engine = CreateBuildingEngine ();
			Assert.Throws<InvalidOperationException> (() => engine.Build (state, state.Regions[0].Name, 0, "Impossible"));
		}

		[Fact]
		public void CanAffordBuilding ()
		{
			GameState state = CreateGameState (camps: 1);
			BuildingEngine engine = CreateBuildingEngine ();
			Assert.False (engine.CanAffordBuilding (state, "Workshop"));
			state = state.WithResources (Create ("Wood", 10.0));
			Assert.True (engine.CanAffordBuilding (state, "Workshop"));
		}

		[Fact]
		public void DestoryValidBuilding ()
		{
			GameState state = CreateGameState (camps: 1);
			BuildingEngine engine = CreateBuildingEngine ();
			state = engine.Destroy (state, state.Regions[0].Name, 0, 0);
			Assert.Empty (state.Regions[0].Areas[0].Buildings);
		}

		[Fact]
		public void DestoryOnlyOneCopy ()
		{
			GameState state = CreateGameState (camps: 2);
			BuildingEngine engine = CreateBuildingEngine ();
			state = engine.Destroy (state, state.Regions[0].Name, 0, 0);
			Assert.Single (state.Regions[0].Areas[0].Buildings);
		}

		[Fact]
		public void DestoryNonExistantBuilding ()
		{
			GameState state = CreateGameState (camps: 1);
			BuildingEngine engine = CreateBuildingEngine ();
			Assert.Throws<InvalidOperationException> (() => engine.Destroy (state, state.Regions[0].Name, 0, 1));
		}

		[Fact]
		public void CanNotDestoryProtectedBuildings ()
		{
			GameState state = CreateGameState (holes: 1);
			BuildingEngine engine = CreateBuildingEngine ();
			Assert.Throws<InvalidOperationException> (() => engine.Destroy (state, state.Regions[0].Name, 0, 0));
		}

		[Fact]
		public void ReturnsOnlyValidBuildingsForArea ()
		{
			GameState state = CreateGameState (camps: 1);
			BuildingEngine engine = CreateBuildingEngine ();
			var buildings = engine.GetValidBuildingsForArea (state, state.Regions[0].Areas[0]);
			Assert.True (buildings.Count > 4);
			Assert.Contains (buildings, x => x == "Gathering Camp");
			Assert.Contains (buildings, x => x == "Workshop");
			Assert.Contains (buildings, x => x == "Smoker");
		}

		[Fact]
		public void AvailableBuildingsMayChangeDueToTechnology ()
		{
			const string extraBuildingJSON = @",{
				""name"": ""RequiresTech"",
				""valid_areas"": [""Plains""],
				""required_technology"": ""Tech""
			}";
			ConfigureCustomJsonPayload (extraBuildingJSON: extraBuildingJSON);

			GameState state = CreateGameState (camps: 1);
			BuildingEngine engine = CreateBuildingEngine ();
			var buildings = engine.GetValidBuildingsForArea (state, state.Regions[0].Areas[0]);
			Assert.DoesNotContain (buildings, x => x == "RequiresTech");

			state = state.WithResearchUnlocks (new string [] { "Tech" });

			buildings = engine.GetValidBuildingsForArea (state, state.Regions[0].Areas[0]);
			Assert.Contains (buildings, x => x == "RequiresTech");
		}

		[Fact]
		public void BuildingsMayRequireFeature ()
		{
			const string extraBuildingJSON = @",{
				""name"": ""RequiresFeature"",
				""required_feature"": ""Feature"",
				""valid_areas"": [ ""Plains"" ]
			}";
			const string extraFeatureJSON = @"{
				""name"": ""Feature""
			}";
			ConfigureCustomJsonPayload (extraBuildingJSON: extraBuildingJSON, extraFeatureJSON: extraFeatureJSON);

			BuildingEngine engine = CreateBuildingEngine ();
			GameState state = CreateGameState (camps: 1);

			var buildings = engine.GetValidBuildingsForArea (state, state.Regions[0].Areas[0]);
			Assert.DoesNotContain (buildings, x => x == "RequiresFeature");

			var firstArea = state.Regions[0].Areas[0].WithFeatures ("Feature".Yield ());
			var areas = state.Regions[0].Areas.Replace (state.Regions[0].Areas[0], firstArea);
			var region = state.Regions[0].WithAreas (areas);
			var regions = state.Regions.Replace (state.Regions[0], region);
			state = state.WithRegions (regions);

			buildings = engine.GetValidBuildingsForArea (state, state.Regions[0].Areas[0]);
			Assert.Contains (buildings, x => x == "RequiresFeature");
		}
	}
}
