using System;

using IncrementalSociety.Generator;
using Xunit;

namespace IncrementalSociety.Tests.Generator
{
	public class NameGeneratorTests
	{
		[Fact]
		public void UsesStartEndByDefault ()
		{
			const string JSON = @"{
				""start"": [ ""Start"" ],
				""end"": [ ""End"" ],
			}";

			var generator = new NameGenerator (JSON);

			for (int i = 0; i < 5; ++i)
				Assert.Equal("StartEnd", generator.Generate ());
		}

		[Fact]
		public void CanUseMiddleOptionally ()
		{
			const string JSON = @"{
				""start"": [ ""Start"" ],
				""middle"": [ ""Middle"" ],
				""end"": [ ""End"" ],
			}";

			var generator = new NameGenerator (JSON);

			int start = 0;
			int middle = 0;
			for (int i = 0; i < 500; ++i) {
				switch (generator.Generate ())
				{
					case "StartEnd":
						start++;
						break;
					case "StartMiddleEnd":
						middle++;
						break;
					default:
						throw new InvalidOperationException ();
				}
			}
			// It is random, but rather unlikley to for a 50% to give us less than 10%
			Assert.True (start > 50);
			Assert.True (middle > 50);
		}

		[Fact]
		public void PreAndPostAreRare ()
		{
			const string JSON = @"{
				""pre"": [ ""Pre"" ],
				""start"": [ ""Start"" ],
				""end"": [ ""End"" ],
				""post"": [ ""Post"" ],
			}";

			var generator = new NameGenerator (JSON);

			int pre = 0;
			int post = 0;
			for (int i = 0; i < 500; ++i)
			{
				string x = generator.Generate ();
				if (x.StartsWith ("Pre"))
					pre++;
				if (x.EndsWith ("Post"))
					post++;				
			}
			// It is random, but rather unlikley to for a 15% to give us less than 1
			Assert.True (pre > 1);
			Assert.True (post > 1);
		}
	}
}
