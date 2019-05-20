using System;
using System.Collections.Generic;
using IncrementalSociety.Json;
using Xunit;

namespace IncrementalSociety.Tests
{
	public class ResourcesTests : ResourceTestBase
	{
		[Fact]
		public void BasicMutability ()
		{
			Resources r = Config.Create ();
			Assert.Equal (0, r["Food"]);

			var builder = r.ToBuilder ();
			builder["Food"] = 10;

			Assert.Equal (10, builder["Food"]);

			Resources r2 = builder.ToResources ();
			Assert.Equal (10, r2["Food"]);
		}

		[Fact]
		public void AddTwoResourcesDifferentItems ()
		{
			var result = CreateBuilder ("Food", 1.0);
			result.Add (Create  ("Water", 1.0));
			Assert.Equal (1, result["Food"]);
			Assert.Equal (1, result["Water"]);

			var chainResult = Create ("Food", 1.0).Add (Create  ("Water", 1.0));
			Assert.Equal (1, chainResult["Food"]);
			Assert.Equal (1, chainResult["Water"]);
		}

		[Fact]
		public void AddTwoResourcesWithSameItems ()
		{
			var result = CreateBuilder ("Food", 1.0);
			result.Add (Create  ("Food", 1.0));
			Assert.Equal (2, result["Food"]);

			var chainResult = Create ("Food", 1.0).Add (Create  ("Food", 1.0));
			Assert.Equal (2, chainResult["Food"]);
		}

		[Fact]
		public void AddTwoResourceOneEmpty ()
		{
			var result = CreateBuilder ("Food", 1.0);
			result.Add (Config.Create ());
			Assert.Equal (1, result["Food"]);

			var chainResult = Create ("Food", 1.0).Add (Config.Create ());
			Assert.Equal (1, chainResult["Food"]);
		}

		[Fact]
		public void AddWithMultiplyTwoResourcesDifferentItems ()
		{
			var result = CreateBuilder ("Food", 1.0);
			result.AddWithMultiply (Create ("Water", 1), .5);
			Assert.Equal (1.0, result["Food"]);
			Assert.Equal (.5, result["Water"]);

			var chainResult = Create ("Food", 1.0).AddWithMultiply (Create ("Water", 1), .5);
			Assert.Equal (1.0, chainResult["Food"]);
			Assert.Equal (.5, chainResult["Water"]);
		}

		[Fact]
		public void AddWithMultiplyTwoResourcesWithSameItems ()
		{
			var result = CreateBuilder ("Food", 1.0);
			result.AddWithMultiply (Create ("Food", 1), .5);
			Assert.Equal (1.5, result["Food"]);

			var chainResult = Create ("Food", 1.0).AddWithMultiply (Create ("Food", 1), .5);
			Assert.Equal (1.5, chainResult["Food"]);
		}

		[Fact]
		public void AddAndMultiplyTwoResourceOneEmpty ()
		{
			var result = CreateBuilder ("Food", 1.0);
			result.AddWithMultiply (Config.Create (), .5);
			Assert.Equal (1, result["Food"]);

			var chainResult = Create ("Food", 1.0).AddWithMultiply (Config.Create (), .5);
			Assert.Equal (1, chainResult["Food"]);
		}

		[Fact]
		public void SubtractTwoResourcesDifferentItems ()
		{
			var result = CreateBuilder ("Food", 1.0);
			result.Subtract (Create ("Water", 1));
			Assert.Equal (1, result["Food"]);
			Assert.Equal (-1, result["Water"]);

			var chainResult = Create ("Food", 1.0).Subtract (Create ("Water", 1));
			Assert.Equal (1, chainResult["Food"]);
			Assert.Equal (-1, chainResult["Water"]);
		}

		[Fact]
		public void SubtractTwoResourcesWithSameItems ()
		{
			var result = CreateBuilder ("Food", 1.0);
			result.Subtract (Create ("Food", 1));
			Assert.Equal (0, result["Food"]);

			var chainResult = Create ("Food", 1.0).Subtract (Create ("Food", 1));
			Assert.Equal (0, chainResult["Food"]);
		}

		[Fact]
		public void SubtractTwoResourceOneEmpty ()
		{
			var result = CreateBuilder ("Food", 1.0);
			result.Subtract (Config.Create ());
			Assert.Equal (1, result["Food"]);

			var chainResult = Create ("Food", 1.0).Subtract (Config.Create ());
			Assert.Equal (1, chainResult["Food"]);
		}

		[Fact]
		public void CompareTwoResourcesDifferentItems ()
		{
			var result = Create ("Food", 1.0);
			Assert.False (result.HasMoreThan (Create ("Water", 1.0)));
		}

		[Fact]
		public void CompareTwoResourcesWithSameItems ()
		{
			var result = Create ("Food", 1.0);
			Assert.True (result.HasMoreThan (Create ("Food", 1.0)));
		}

		[Fact]
		public void CompareTwoResourceOneEmpty ()
		{
			var result = Create ("Food", 1.0);
			Assert.True (result.HasMoreThan (Config.Create ()));
		}

		[Fact]
		public void CompareTwoResourceOneNegative ()
		{
			var result = Create ("Food", -1.0);
			Assert.True (result.HasMoreThan (Create ("Food", -10.0)));
		}

		[Fact]
		public void Multiply ()
		{
			var result = CreateBuilder ("Food", 1.0);
			result["Water"] = 2;
			result.Multiply (2.5);
			Assert.Equal (2.5, result["Food"], 1);
			Assert.Equal (5, result["Water"], 1);

			result = CreateBuilder ("Food", 1.0);
			result["Water"] = 2;
			var builder = result.ToResources ();
			builder = builder.Multiply (2.5);
			Assert.Equal (2.5, builder["Food"], 1);
			Assert.Equal (5, builder["Water"], 1);
		}
	}
}
