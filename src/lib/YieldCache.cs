using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using IncrementalSociety.Json;
using IncrementalSociety.Utilities;

namespace IncrementalSociety
{
	public class YieldCache
	{
		Dictionary<Yield, Resources> YieldLookup;
		Dictionary<ConversionYield, Resources> ConversionYieldLookup;
		ResourceConfig ResourceConfig;

		public YieldCache (ResourceConfig resourceConfig)
		{
			ResourceConfig = resourceConfig;

			YieldLookup = new Dictionary<Yield, Resources> ();
			ConversionYieldLookup = new Dictionary<ConversionYield, Resources> ();
		}

		public Resources From (Yield yield)
		{
			if (YieldLookup.TryGetValue (yield, out Resources value))
			{
				return value;
			}
			else
			{
				var newValue = Convert (yield);
				YieldLookup[yield] = newValue;
				return newValue;
			}
		}

		public Resources From (ConversionYield conversionYield)
		{
			if (ConversionYieldLookup.TryGetValue (conversionYield, out Resources value))
			{
				return value;
			}
			else
			{
				var newValue = Convert (conversionYield);
				ConversionYieldLookup[conversionYield] = newValue;
				return newValue;
			}
		}

		// These public APIs use the cache
		public Resources Total (Yield yield)
		{
			var resources = ResourceConfig.CreateBuilder ();
			resources.Add (From (yield));
			return resources.ToResources ();
		}

		public Resources Total (IEnumerable <Yield> yields)
		{
			var resources = ResourceConfig.CreateBuilder ();
			foreach (var yield in yields.AsNotNull ())
				resources.Add (From (yield));
			return resources.ToResources ();
		}

		// These internal APIs do _not_ use the cache
		Resources Convert (Yield yield)
		{
			var resources = ResourceConfig.CreateBuilder ();
			resources[yield.Name] = yield.Amount;
			return resources.ToResources ();
		}

		Resources Convert (ConversionYield conversionYield)
		{
			var resources = ResourceConfig.CreateBuilder ();
			foreach (var cost in conversionYield.Cost.AsNotNull ())
				resources[cost.Name] = cost.Amount * -1;
			foreach (var provide in conversionYield.Provides.AsNotNull ())
				resources[provide.Name] = provide.Amount;
			return resources.ToResources ();
		}
	}
}
