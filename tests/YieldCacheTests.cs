using System;
using IncrementalSociety.Json;
using Xunit;

namespace IncrementalSociety.Tests
{
	public class YieldCacheTests
	{
		[Fact]
		public void DuplicateItemsYield ()
		{
			YieldCache cache = new YieldCache ();
			var first = cache.From (new Yield { Name = "A", Amount = 1 });
			var second = cache.From (new Yield { Name = "A", Amount = 1 });
			Assert.Equal (first, second);
		}

		[Fact]
		public void DuplicateItemsYieldConversion ()
		{
			YieldCache cache = new YieldCache ();
			var first = cache.From (new ConversionYield () { Name = "Test", Cost = new Yield[] { new Yield { Name = "A", Amount = 1 } } });
			var second = cache.From (new ConversionYield () { Name = "Test", Cost = new Yield[] { new Yield { Name = "A", Amount = 1 } } });
			Assert.Equal (first, second);
		}
		
		[Fact]
		public void ConversionYieldCostsAreNegative ()
		{
			YieldCache cache = new YieldCache ();
			var yield = cache.From (new ConversionYield () { 
					Name = "Test",
					Cost = new Yield[] { new Yield { Name = "A", Amount = 1 } },
					Provides = new Yield[] { new Yield { Name = "B", Amount = .5 } }
			});
			Assert.Equal (-1, yield["A"]);
			Assert.Equal (.5, yield["B"]);
		}

	}
}
