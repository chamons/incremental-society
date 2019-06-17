using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.Text;

using IncrementalSociety.Utilities;

namespace IncrementalSociety.Generator
{
	public class NameData
	{
		[JsonProperty("pre")]
		public string [] Prefixes { get; set; }

		[JsonProperty("start")]
		public string[] Starts { get; set; }

		[JsonProperty("middle")]
		public string[] Middles { get; set; }

		[JsonProperty("end")]
		public string[] Ends { get; set; }

		[JsonProperty("post")]
		public string[] Posts { get; set; }
	}

	public class NameGenerator
	{
		NameData Names;
		Random Random;

		public NameGenerator (Json.JsonLoader loader) : this (loader.RegionNameJSON)
		{
		}

		public NameGenerator (string nameJson)
		{
			Names = JsonConvert.DeserializeObject<NameData>(nameJson);
			Random = new Random ();
		}

		string PickItem (string [] list)
		{
			if (list == null)
				return "";

			return list[Random.Next (list.Length)];
		}

		public string Generate ()
		{
			StringBuilder name = new StringBuilder ();

			if (Random.WithChance (15))
				name.Append (PickItem (Names.Prefixes));

			name.Append (PickItem (Names.Starts));

			if (Random.WithChance (50))
				name.Append (PickItem (Names.Middles));

			name.Append (PickItem (Names.Ends));

			if (Random.WithChance (15))
				name.Append (PickItem (Names.Posts));

			return name.ToString ();
		}
	}
}
