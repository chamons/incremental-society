using System.Collections.Immutable;
using IncrementalSociety.Utilities;
using Xunit;

namespace IncrementalSociety.Tests
{
	public class ResourceExtensionTests
	{
		[Fact]
		public void AmountOf ()
		{
			var result = Immutable.CreateBuilderDictionary ("Food", 1.0);
			Assert.Equal (1, result.AmountOf ("Food"));
			Assert.Equal (0, result.AmountOf ("Water"));
		}

		[Fact]
		public void AddTwoResourcesDifferentItems ()
		{
			var result = Immutable.CreateBuilderDictionary ("Food", 1.0);
			result.Add (Immutable.CreateDictionary ("Water", 1.0));
			Assert.Equal (1, result["Food"]);
			Assert.Equal (1, result["Water"]);
		}

		[Fact]
		public void AddTwoResourcesWithSameItems ()
		{
			var result = Immutable.CreateBuilderDictionary ("Food", 1.0);
			result.Add (Immutable.CreateDictionary ("Food", 1.0));
			Assert.Equal (2, result["Food"]);
		}

		[Fact]
		public void AddTwoResourceOneEmpty ()
		{
			var result = Immutable.CreateBuilderDictionary ("Food", 1.0);
			result.Add (ImmutableDictionary<string, double>.Empty);
			Assert.Equal (1, result["Food"]);
		}

		[Fact]
		public void AddWithMultiplyTwoResourcesDifferentItems ()
		{
			var result = Immutable.CreateBuilderDictionary ("Food", 1.0);
			result.AddWithMultiply (Immutable.CreateDictionary ("Water", 1.0), .5);
			Assert.Equal (1, result["Food"]);
			Assert.Equal (.5, result["Water"]);
		}

		[Fact]
		public void AddWithMultiplyTwoResourcesWithSameItems ()
		{
			var result = Immutable.CreateBuilderDictionary ("Food", 1.0);
			result.AddWithMultiply (Immutable.CreateDictionary ("Food", 1.0), .5);
			Assert.Equal (1.5, result["Food"]);
		}

		[Fact]
		public void AddAndMultiplyTwoResourceOneEmpty ()
		{
			var result = Immutable.CreateBuilderDictionary ("Food", 1.0);
			result.AddWithMultiply (ImmutableDictionary<string, double>.Empty, .5);
			Assert.Equal (1, result["Food"]);
		}

		[Fact]
		public void SubtractTwoResourcesDifferentItems ()
		{
			var result = Immutable.CreateBuilderDictionary ("Food", 1.0);
			result .Subtract (Immutable.CreateDictionary ("Water", 1.0));
			Assert.Equal (1, result["Food"]);
			Assert.Equal (-1, result["Water"]);
		}

		[Fact]
		public void SubtractTwoResourcesWithSameItems ()
		{
			var result = Immutable.CreateBuilderDictionary ("Food", 1.0);
			result.Subtract (Immutable.CreateDictionary ("Food", 1.0));
			Assert.Equal (0, result["Food"]);
		}

		[Fact]
		public void SubtractTwoResourceOneEmpty ()
		{
			var result = Immutable.CreateBuilderDictionary ("Food", 1.0);
			result.Subtract (ImmutableDictionary<string, double>.Empty);
			Assert.Equal (1, result["Food"]);
		}

		[Fact]
		public void CompareTwoResourcesDifferentItems ()
		{
			var result = Immutable.CreateDictionary ("Food", 1.0);
			Assert.False (result.HasMoreThan (Immutable.CreateDictionary ("Water", 1.0)));
		}

		[Fact]
		public void CompareTwoResourcesWithSameItems ()
		{
			var result = Immutable.CreateDictionary ("Food", 1.0);
			Assert.True (result.HasMoreThan (Immutable.CreateDictionary ("Food", 1.0)));
		}

		[Fact]
		public void CompareTwoResourceOneEmpty ()
		{
			var result = Immutable.CreateDictionary ("Food", 1.0);
			Assert.True (result.HasMoreThan (ImmutableDictionary<string, double>.Empty));
		}

		[Fact]
		public void CompareTwoResourceOneNegative ()
		{
			var result = Immutable.CreateDictionary ("Food", -1.0);
			Assert.True (result.HasMoreThan (Immutable.CreateDictionary ("Food", -10.0)));
		}

		[Fact]
		public void Multiply ()
		{
			var result = Immutable.CreateBuilderDictionary ("Food", 1.0);
			result.Add ("Water", 2);
			result.Multiply (2.5);
			Assert.Equal (2.5, result["Food"], 1);
			Assert.Equal (5, result["Water"], 1);
		}
	}
}
