using System;
using System.Collections.Generic;
using IncrementalSociety.Json;
using Xunit;

namespace IncrementalSociety.Tests
{
	public class ResourcesTests
	{
		[Fact]
		public void BasicMutability ()
		{
			var config = new ResourceConfig (new List<string> () { "Foo", "Bar" });

			Resources r = config.Create ();
			Assert.Equal (0, r["Foo"]);

			var builder = r.ToBuilder ();
			builder["Foo"] = 10;

			Assert.Equal (10, builder["Foo"]);

			Resources r2 = builder.ToResources ();
			Assert.Equal (10, r2["Foo"]);
		}
	}
}