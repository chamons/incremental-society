using System;
using System.Collections.Generic;
using IncrementalSociety.Json;
using Xunit;

namespace IncrementalSociety.Tests
{
	public class EdictCooldownTests : ResourceTestBase
	{
		[Fact]
		public void BasicMutability ()
		{
			const string extraEdictsJSON = @"
				{
					""name"": ""First""
				},
				{
					""name"": ""Second""
				}
			";

			ConfigureCustomJsonPayload (extraEdictsJSON: extraEdictsJSON);


			EdictCooldown cooldown = EdictConfig.Create ();
			Assert.Equal (0, cooldown["First"]);

			cooldown = cooldown.Add ("First", 10);
			Assert.Equal (10, cooldown["First"]);

			cooldown = cooldown.Add ("Second", 10);
			Assert.Equal (10, cooldown["First"]);
			Assert.Equal (10, cooldown["Second"]);

			cooldown = cooldown.Tick ();
			Assert.Equal (9, cooldown["First"]);
			Assert.Equal (9, cooldown["Second"]);
		}
	}
}
