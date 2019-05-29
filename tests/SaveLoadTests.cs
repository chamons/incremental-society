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
			// This is some ugly static state we need since we do not serialize our resource json in save games
			Resources.SaveLoadConfig = Config;
			EdictCooldown.SaveLoadConfig = EdictConfig;

			GameState state = CreateGameState (camps: 1);
			state = state.WithResources (Create ("Food", 100));
			string serialized = JsonConvert.SerializeObject (state);

			GameState inflatedState = JsonConvert.DeserializeObject<GameState> (serialized);
			Assert.Equal (100, inflatedState.Resources["Food"]);
		}
	}
}
