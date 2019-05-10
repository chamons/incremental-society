using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using IncrementalSociety.Json;
using IncrementalSociety.Utilities;

namespace IncrementalSociety
{
	public class YieldCache
	{
		Dictionary<Yield, ImmutableDictionary<string, double>> YieldLookup;
		Dictionary<ConversionYield, ImmutableDictionary<string, double>> ConversionYieldLookup;

		public YieldCache ()
		{
			YieldLookup = new Dictionary<Yield, ImmutableDictionary<string, double>> ();
			ConversionYieldLookup = new Dictionary<ConversionYield, ImmutableDictionary<string, double>> ();
		}

		public ImmutableDictionary<string, double> From (Yield yield)
		{
			if (YieldLookup.TryGetValue (yield, out ImmutableDictionary<string, double> value))
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

		public ImmutableDictionary<string, double> From (ConversionYield conversionYield)
		{
			if (ConversionYieldLookup.TryGetValue (conversionYield, out ImmutableDictionary<string, double> value))
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

		public ImmutableDictionary<string, double> Total (Yield[] yields)
		{
			var resources = ImmutableDictionary.CreateBuilder<string, double> ();
			foreach (var yield in yields.AsNotNull ())
				resources.Add (From (yield));
			return resources.ToImmutable ();
		}

		static ImmutableDictionary<string, double> Convert (Yield yield)
		{
			var resources = ImmutableDictionary.CreateBuilder<string, double> ();
			resources.Add (yield.Name, yield.Amount);
			return resources.ToImmutable ();
		}

		static ImmutableDictionary<string, double> Convert (ConversionYield conversionYield)
		{
			var resources = ImmutableDictionary.CreateBuilder<string, double> ();
			foreach (var cost in conversionYield.Cost.AsNotNull ())
				resources.Add (cost.Name, cost.Amount * -1);
			foreach (var provide in conversionYield.Provides.AsNotNull ())
				resources.Add (provide.Name, provide.Amount);
			return resources.ToImmutable ();
		}
	}
}
