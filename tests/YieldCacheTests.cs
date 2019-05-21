using System;
using System.Collections.Generic;

using Xunit;
using IncrementalSociety.Json;

namespace IncrementalSociety.Tests
{
	public class YieldCacheTests : ResourceTestBase
	{
		[Fact]
		public void DuplicateItemsYield ()
		{
			YieldCache cache = new YieldCache (Config);
			var first = cache.From (new Yield { Name = "Food", Amount = 1 });
			var second = cache.From (new Yield { Name = "Food", Amount = 1 });
			Assert.Equal (first, second);
		}

		[Fact]
		public void DuplicateItemsYieldConversion ()
		{
			YieldCache cache = new YieldCache (Config);
			var first = cache.From (new ConversionYield () { Name = "Test", Cost = new Yield[] { new Yield { Name = "Food", Amount = 1 } } });
			var second = cache.From (new ConversionYield () { Name = "Test", Cost = new Yield[] { new Yield { Name = "Food", Amount = 1 } } });
			Assert.Equal (first, second);
		}

		[Fact]
		public void ConversionYieldCostsAreNegative ()
		{
			YieldCache cache = new YieldCache (Config);
			var yield = cache.From (new ConversionYield () {
					Name = "Test",
					Cost = new Yield[] { new Yield { Name = "Food", Amount = 1 } },
					Provides = new Yield[] { new Yield { Name = "Water", Amount = .5 } }
			});
			Assert.Equal (-1, yield["Food"]);
			Assert.Equal (.5, yield["Water"]);
		}

	}
}
