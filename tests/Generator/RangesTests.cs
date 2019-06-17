using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

using Xunit;

using IncrementalSociety.Generator;
using IncrementalSociety.Tests;

namespace IncrementalSociety.Generator.Tests
{
	public class RangesTests
	{
		[Fact]
		public void FindValueInRange ()
		{
			var range = new Ranges (new (string Name, double Value) [] {("A", .6), ("B", .4), ("C", 0)});
			Assert.Equal ("A", range[0]);
			Assert.Equal ("A", range[.6]);
			Assert.Equal ("B", range[.7]);
			Assert.Equal ("B", range[.9]);
		}

		[Fact]
		public void ThrowsIfOutOfRange ()
		{
			var range = new Ranges (new (string Name, double Value) [] {("A", .6), ("B", .4), ("C", 0)});
			Assert.Throws<InvalidOperationException> (() => range[-1]);
			Assert.Throws<InvalidOperationException> (() => range[2]);
		}

		[Fact]
		public void ThrowsIfValuesDoNotAddUp ()
		{
			Assert.Throws <InvalidOperationException> (() => new Ranges (new (string Name, double Value) [] {("A", .2)}));
			Assert.Throws <InvalidOperationException> (() => new Ranges (new (string Name, double Value) [] {("A", .2), ("B", 1)}));

		}
	}
}