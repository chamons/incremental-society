using System;
using System.Collections.Generic;
using System.Text;

namespace IncrementalSociety.Utilities
{
	public static class RandomExtensions
	{
		public static bool WithChance (this Random r, double chance)
		{
			return (r.NextDouble () * 100) < chance;
		}

		public static T RandomItem<T> (this Random r, List<T> list)
		{
			return list[r.Next (0, list.Count)];
		}
	}
}
