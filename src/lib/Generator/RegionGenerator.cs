using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

using IncrementalSociety.Population;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

namespace IncrementalSociety.Generator
{
	public class RegionGenerator
	{
		JsonLoader Json;

		public RegionGenerator (JsonLoader json)
		{
			Json = json;
		}
	}
}
