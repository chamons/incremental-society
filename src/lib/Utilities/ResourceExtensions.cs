using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Text;

namespace IncrementalSociety.Utilities
{
	public static class ResourceExtensions
	{
		public static double AmountOf (this IDictionary<string, double> resources, string resourceName)
		{
			return resources.ContainsKey (resourceName) ? resources[resourceName] : 0;
		}

		public static void Add (this ImmutableDictionary<string, double>.Builder left, IDictionary<string, double> right)
		{
			foreach (var resourceName in left.Keys.Union (right.Keys).ToList ())
			{
				double leftValue = left.AmountOf (resourceName);
				double rightValue = right.AmountOf (resourceName);
				left[resourceName] = leftValue + rightValue;
			}
		}

		public static void Subtract (this ImmutableDictionary<string, double>.Builder left, IDictionary<string, double> right)
		{
			foreach (var resourceName in left.Keys.Union (right.Keys).ToList ())
			{
				double leftValue = left.AmountOf (resourceName);
				double rightValue = right.AmountOf (resourceName);
				left[resourceName] = leftValue - rightValue;
			}
		}

		public static void Multiply (this ImmutableDictionary<string, double>.Builder left, double right)
		{
			if (right == 1)
				return;
			foreach (var resourceName in left.Keys.ToList ()) 
				left[resourceName] = left[resourceName] * right;
		}

		public static bool HasMoreThan (this ImmutableDictionary<string, double> left, IDictionary<string, double> right)
		{
			ImmutableDictionary<string, double>.Builder remain = left.ToBuilder ();
			remain.Subtract (right);
			if (right.Keys.Any (x => remain[x] < 0))
				return false;
			return true;
		}
	}
}
