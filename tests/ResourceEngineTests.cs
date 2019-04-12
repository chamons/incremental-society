using System.Collections.Immutable;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;
using Xunit;

namespace IncrementalSociety.Tests
{

    public class ResourceEngineTests
    {
        [Fact]
        public void AddTwoResourcesDifferentItems ()
        {
            var result = ResourceEngine.AddResources (Immutable.CreateDictionary ("A", 1), Immutable.CreateDictionary ("B", 1));
            Assert.Equal (1, result["A"]);
            Assert.Equal (1, result["B"]);
        }

        [Fact]
        public void AddTwoResourcesWithSameItems ()
        {
            var result = ResourceEngine.AddResources (Immutable.CreateDictionary ("A", 1), Immutable.CreateDictionary ("A", 1));
            Assert.Equal (2, result["A"]);
        }

        [Fact]
        public void AddTwoResourceOneEmpty ()
        {
            var result = ResourceEngine.AddResources (Immutable.CreateDictionary ("A", 1), ImmutableDictionary<string, int>.Empty);
            Assert.Equal (1, result["A"]);
        }
    }
}
