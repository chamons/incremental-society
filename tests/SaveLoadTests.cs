using IncrementalSociety.Model;
using Newtonsoft.Json;
using Xunit;

namespace IncrementalSociety.Tests
{
	public class SaveLoadTests : ResourceTestBase
	{
		[Fact]
		public void SaveLoadRoundTrip ()
		{
			GameState state = Factories.CreateGameState (camps: 1);
			state = state.WithResources (Create ("Food", 100));
			string serialized = JsonConvert.SerializeObject (state);

			GameState inflatedState = JsonConvert.DeserializeObject<GameState> (serialized);
			Assert.Equal (100, inflatedState.Resources["Food"]);
		}
	}
}
