using System;
using System.Collections.Generic;
using System.Text;

namespace IncrementalSociety.Utilities
{
	public static class RandomExtensions
	{
		public static bool WithChance (this Random r, int chance)
		{
			return (r.NextDouble () * 100) < chance;
		}
	}
}
