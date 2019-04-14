using System.Collections.Immutable;
using IncrementalSociety.Model;
using System.Linq;
using System.Collections.Generic;

namespace IncrementalSociety
{
    public static class ResourceEngine
    {
        public static ImmutableDictionary<string, int> CalculateAdditionalNextTick (GameState state)
        {
			foreach (var region in state.Regions)
			{
				
			}
            return ImmutableDictionary.Create<string, int> ();
        }

        public static ImmutableDictionary<string, int> AddResources (ImmutableDictionary<string, int> left, ImmutableDictionary<string, int> right)
        {
			Dictionary<string, int> newValues = new Dictionary<string, int> ();
            foreach (var resourceName in left.Keys.Union (right.Keys))
            {
				int leftValue = left.ContainsKey (resourceName) ? left[resourceName] : 0;
				int rightValue = right.ContainsKey (resourceName) ? right[resourceName] : 0;
				newValues[resourceName] = leftValue + rightValue;
			}

            return newValues.ToImmutableDictionary ();
        }
    }
}
