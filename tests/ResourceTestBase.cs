using System;
using System.Collections.Generic;
using IncrementalSociety.Json;
using Xunit;

namespace IncrementalSociety.Tests
{
	public abstract class ResourceTestBase
	{
		protected ResourceConfig Config;

		protected ResourceTestBase ()
		{
			Config = new ResourceConfig (new List<string> () { "Food", "Water" });
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