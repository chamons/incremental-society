using System;
using Xunit;

namespace IncrementalSociety.Tests
{
    public class ResourcesTest
    {
        [Fact]
        public void FindsExpectedResourceBundles ()
        {
            var resources = Resources.Load ();
            Assert.NotNull (resources.ActionsJSON);
            Assert.NotNull (resources.BuildingsJSON);
            Assert.NotNull (resources.GameJSON);
            Assert.NotNull (resources.RegionsJSON);
            Assert.NotNull (resources.ResourcesJSON);
        }
    }
}
