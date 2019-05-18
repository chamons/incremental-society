using System;
using System.Collections.Immutable;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;
using Xunit;

namespace IncrementalSociety.Tests
{
	public class BuildingEngineTests
	{
		[Fact]
		public void BuildValidBuilding ()
		{
			GameState state = Factories.CreateGameState (camps: 1);
			state = state.WithResources (Immutable.CreateBuilderDictionary ("Wood", 10.0));
			BuildingEngine engine = Factories.CreateBuildingEngine ();
			state = engine.Build (state, state.Regions[0].Name, 0, "Workshop");
			Assert.Equal (2, state.Regions[0].Areas[0].Buildings.Length);
			Assert.Equal (0, state.Resources["Wood"]);
		}

		[Fact]
		public void BuildBuildingWhereNoRoom ()
		{
			GameState state = Factories.CreateGameState (camps: 3);
			state = state.WithResources (Immutable.CreateBuilderDictionary ("Wood", 10.0));
			BuildingEngine engine = Factories.CreateBuildingEngine ();

			Assert.Throws<InvalidOperationException> (() => engine.Build (state, state.Regions[0].Name, 0, "Workshop"));
		}

		[Fact]
		public void BuildBuildingInvalidRegionType ()
		{
			GameState state = Factories.CreateGameState (camps: 1);
			state = state.WithResources (Immutable.CreateBuilderDictionary ("Wood", 10.0));
			BuildingEngine engine = Factories.CreateBuildingEngine ();
			Assert.Throws<InvalidOperationException> (() => engine.Build (state, state.Regions[0].Name, 0, "Mine"));
		}

		[Fact]
		public void BuildBuildingValidAnywhere ()
		{
			GameState state = Factories.CreateGameState (camps: 1);
			state = state.WithResources (Immutable.CreateBuilderDictionary ("Wood", 10.0));
			BuildingEngine engine = Factories.CreateBuildingEngine ();
			engine.Build (state, state.Regions[0].Name, 0, "Any");
		}

		[Fact]
		public void BuildBuildingWithoutResourcs ()
		{
			GameState state = Factories.CreateGameState (camps: 1);
			BuildingEngine engine = Factories.CreateBuildingEngine ();
			Assert.Throws<InvalidOperationException> (() => engine.Build (state, state.Regions[0].Name, 0, "Workshop"));
		}

		[Fact]
		public void CannotBuildBuildingMarkedUnable ()
		{
			GameState state = Factories.CreateGameState ();
			BuildingEngine engine = Factories.CreateBuildingEngine ();
			Assert.Throws<InvalidOperationException> (() => engine.Build (state, state.Regions[0].Name, 0, "Impossible"));
		}

		[Fact]
		public void CanAffordBuilding ()
		{
			GameState state = Factories.CreateGameState (camps: 1);
			BuildingEngine engine = Factories.CreateBuildingEngine ();
			Assert.False (engine.CanAffordBuilding (state, "Workshop"));
			state = state.WithResources (Immutable.CreateBuilderDictionary ("Wood", 10.0));
			Assert.True (engine.CanAffordBuilding (state, "Workshop"));
		}

		[Fact]
		public void DestoryValidBuilding ()
		{
			GameState state = Factories.CreateGameState (camps: 1);
			BuildingEngine engine = Factories.CreateBuildingEngine ();
			state = engine.Destroy (state, state.Regions[0].Name, 0, 0);
			Assert.Empty (state.Regions[0].Areas[0].Buildings);
		}

		[Fact]
		public void DestoryOnlyOneCopy ()
		{
			GameState state = Factories.CreateGameState (camps: 2);
			BuildingEngine engine = Factories.CreateBuildingEngine ();
			state = engine.Destroy (state, state.Regions[0].Name, 0, 0);
			Assert.Single (state.Regions[0].Areas[0].Buildings);
		}

		[Fact]
		public void DestoryNonExistantBuilding ()
		{
			GameState state = Factories.CreateGameState (camps: 1);
			BuildingEngine engine = Factories.CreateBuildingEngine ();
			Assert.Throws<InvalidOperationException> (() => engine.Destroy (state, state.Regions[0].Name, 0, 1));
		}
		
		[Fact]
		public void CanNotDestoryProtectedBuildings ()
		{
			GameState state = Factories.CreateGameState (holes: 1);
			BuildingEngine engine = Factories.CreateBuildingEngine ();
			Assert.Throws<InvalidOperationException> (() => engine.Destroy (state, state.Regions[0].Name, 0, 0));
		}

		[Fact]
		public void ReturnsOnlyValidBuildingsForArea ()
		{
			GameState state = Factories.CreateGameState (camps: 1);
			BuildingEngine engine = Factories.CreateBuildingEngine ();
			var buildings = engine.GetValidBuildingsForArea (state.Regions[0].Areas[0]);
			Assert.True (buildings.Count > 5);
			Assert.Contains (buildings, x => x == "Gathering Camp");
			Assert.Contains (buildings, x => x == "Workshop");
			Assert.Contains (buildings, x => x == "Smoker");
		}
	}
}
