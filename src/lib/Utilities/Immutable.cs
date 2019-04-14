using System.Collections.Generic;
using System.Collections.Immutable;

namespace IncrementalSociety.Utilities
{
    public static class Immutable
    {
        public static ImmutableArray<T> Create<T> (params T[] items)
        {
            return items.ToImmutableArray ();
        }

        public static ImmutableDictionary<T, U> CreateDictionary<T, U> (T key, U value)
        {
            var d = new Dictionary<T, U> ();
            d.Add (key, value);
            return d.ToImmutableDictionary ();
        }

		public static ImmutableDictionary<T, U>.Builder CreateBuilderDictionary<T, U> (T key, U value)
		{
			var d = ImmutableDictionary.CreateBuilder<T, U> ();
			d[key] = value;
			return d;
		}
	}
}
