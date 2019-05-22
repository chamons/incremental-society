using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Text;
using IncrementalSociety.Json;
using IncrementalSociety.Model;
using IncrementalSociety.Utilities;

namespace IncrementalSociety
{
	public class ResearchEngine
	{
		JsonLoader Json;

		public ResearchEngine (JsonLoader json)
		{
			Json = json;
		}
	}
}
