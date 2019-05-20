using System;
using System.Collections.Generic;
using System.Linq;
using IncrementalSociety.Json;
using Xunit;

namespace IncrementalSociety.Tests
{
	public abstract class ResourceTestBase
	{
		protected ResourceConfig Config;

		protected ResourceTestBase ()
		{
			JsonLoader loader = new JsonLoader ("", "", "", Factories.ResourceJSON, validate: false);
			Config = new ResourceConfig (loader.Resources.Resources.Select (x => x.Name));
		}

		protected Resources.Builder CreateBuilder (string resource, double amount)
		{
			var builder = Config.CreateBuilder ();
			builder[resource] = amount;
			return builder;
		}

		protected Resources Create (string resource, double amount) => CreateBuilder (resource, amount).ToResources ();
	}
}