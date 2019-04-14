using System;
using System.Collections.Generic;
using System.Linq;

namespace IncrementalSociety.Utilities
{
	public static class EnumerableExtensions
	{
		public static IEnumerable<T> AsNotNull<T> (this IEnumerable<T> original)
		{
			return original ?? Enumerable.Empty<T> ();
		}
	}
}
