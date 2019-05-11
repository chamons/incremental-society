using System;
using System.Collections.Generic;
using System.Text;

namespace IncrementalSociety.Utilities
{
	static class MathUtilities
	{
		public static double Clamp (double x, double min, double max) => Math.Min(Math.Max(x, min), max);
	}
}