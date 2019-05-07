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
			BuildingEngine engine = Factories.CreateBuildingEngine ();
			state = engine.Build (state, state.Regions[0].Name, 0, "Workshop");
			Assert.Equal (2, state.Regions[0].Areas[0].Buildings.Length); 
		}
		
		[Fact]
		public void BuildBuildingWhereNoRoom ()
		{
			GameState state = Factories.CreateGameState (camps: 2);
			BuildingEngine engine = Factories.CreateBuildingEngine ();
			
			Assert.Throws<InvalidOperationException> (() => engine.Build (state, state.Regions[0].Name, 0, "Workshop"));
		}

		[Fact]
		public void BuildBuildingInvalidRegionType ()
		{
			GameState state = Factories.CreateGameState (camps: 1);
			BuildingEngine engine = Factories.CreateBuildingEngine ();
			Assert.Throws<InvalidOperationException> (() => engine.Build (state, state.Regions[0].Name, 0, "Mine"));
		}
		
		[Fact]
		public void BuildBuildingInvalidBuildingName ()
		{
			GameState state = Factories.CreateGameState (camps: 1);
			BuildingEngine engine = Factories.CreateBuildingEngine ();
			Assert.Throws<InvalidOperationException> (() => engine.Build (state, state.Regions[0].Name, 0, "Invalid"));
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
		public void ReturnsOnlyValidBuildingsForArea ()
		{
			GameState state = Factories.CreateGameState (camps: 1);
			BuildingEngine engine = Factories.CreateBuildingEngine ();
			var buildings = engine.GetValidBuildingsForArea (state.Regions[0].Areas[0]);
			Assert.Equal (2, buildings.Count);
			Assert.Contains (buildings, x => x.BuildingName == "Gathering Camp");
			Assert.Contains (buildings, x => x.BuildingName == "Workshop");
		}
	}
}
