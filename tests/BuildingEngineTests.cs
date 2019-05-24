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
			GameState state = CreateGameState (camps: 1);
			state = state.WithResources (Create ("Wood", 10.0));
			BuildingEngine engine = CreateBuildingEngine ();
			Assert.Throws<InvalidOperationException> (() => engine.Build (state, state.Regions[0].Name, 0, "Mine"));
		}

		[Fact]
		public void BuildBuildingValidAnywhere ()
		{
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
			var buildings = engine.GetValidBuildingsForArea (state.Regions[0].Areas[0]);
			Assert.True (buildings.Count > 5);
			Assert.Contains (buildings, x => x == "Gathering Camp");
			Assert.Contains (buildings, x => x == "Workshop");
			Assert.Contains (buildings, x => x == "Smoker");
		}
	}
}
