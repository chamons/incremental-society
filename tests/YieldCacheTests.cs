using System;
using Xunit;

namespace IncrementalSociety.Tests
{
	public class YieldCacheTests
	{
		[Fact]
		public void DuplicateItems ()
		{
			YieldCache cache = new YieldCache ();
			cache.From (new Json.Yield { Name = "A", Amount = 1 });
		}
	}
}
