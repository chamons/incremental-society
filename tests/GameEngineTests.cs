using System;
using System.Collections.Immutable;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;
using Xunit;

namespace IncrementalSociety.Tests
{
	public class GameEngineTests
	{
		[Fact]
		public void BuildValidBuilding ()
		{
			GameState state = Factories.CreateGameStateWithOneCamp ();
			GameEngine engine = Factories.CreateGameEngine ();
			state = engine.ApplyAction (state, "Build District", new string [] { state.Regions[0].Name, 0.ToString (), "Workshop" });
			Assert.Equal (2, state.Regions[0].Areas[0].Buildings.Length); 
		}
		
		[Fact]
		public void BuildBuildingWhereNoRoom ()
		{
			GameState state = Factories.CreateGameStateFullOfCamps ();
			GameEngine engine = Factories.CreateGameEngine ();
			
			Assert.Throws<InvalidOperationException> (() => engine.ApplyAction (state, "Build District", new string [] { state.Regions[0].Name, 0.ToString (), "Workshop" }));
		}

		[Fact]
		public void BuildBuildingInvalidRegionType ()
		{
			GameState state = Factories.CreateGameStateWithOneCamp ();
			GameEngine engine = Factories.CreateGameEngine ();
			Assert.Throws<InvalidOperationException> (() => engine.ApplyAction (state, "Build District", new string [] { state.Regions[0].Name, 0.ToString (), "Mine" }));
		}

		[Fact]
		public void DestoryValidBuilding ()
		{
			GameState state = Factories.CreateGameStateWithOneCamp ();
			GameEngine engine = Factories.CreateGameEngine ();
			state = engine.ApplyAction (state, "Destory District", new string [] { state.Regions[0].Name, 0.ToString () });
			Assert.Empty (state.Regions[0].Areas[0].Buildings); 
		}
		
		[Fact]
		public void DestoryNonExistantBuilding ()
		{
			GameState state = Factories.CreateGameStateWithOneCamp ();
			GameEngine engine = Factories.CreateGameEngine ();
			Assert.Throws<InvalidOperationException> (() => engine.ApplyAction (state, "Destory District", new string [] { state.Regions[0].Name, 1.ToString () }));
		}
	}
}
