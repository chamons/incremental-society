using System;
using System.Collections.Generic;
using System.Text;

namespace IncrementalSociety.Utilities
{
	static class MathUtilities
	{
		public static double Clamp (double x, double min, double max) => Math.Min(Math.Max(x, min), max);

		public static bool HasValue(this double value) => !Double.IsNaN(value) && !Double.IsInfinity(value);

		public static bool Is (this double a, double b, double tolerance = .001)
		{
			return Math.Abs (a - b) < tolerance;
		}
	}
}
