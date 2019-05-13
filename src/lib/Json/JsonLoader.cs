using IncrementalSociety.Json;
using IncrementalSociety.Utilities;
using Newtonsoft.Json;
using System;
using System.IO;
using System.Linq;
using System.Reflection;

namespace IncrementalSociety.Json
{
	public class JsonLoader
	{
		public string BuildingsJSON { get; }
		public BuildingDeclarations Buildings { get; }

		public string GameJSON { get; }
		public GameDeclarations Game { get; }

		public string RegionsJSON { get; }
		public RegionDeclarations Regions { get; }

		public string ResourcesJSON { get; }
		public ResourceDeclarations Resources { get; }

		public JsonLoader (string buildings, string game, string regions, string resources)
		{
			BuildingsJSON = buildings;
			Buildings = JsonConvert.DeserializeObject<BuildingDeclarations> (BuildingsJSON);

			GameJSON = game;
			Game = JsonConvert.DeserializeObject<GameDeclarations> (GameJSON);

			RegionsJSON = regions;
			Regions = JsonConvert.DeserializeObject<RegionDeclarations> (RegionsJSON);

			ResourcesJSON = resources;
			Resources = JsonConvert.DeserializeObject<ResourceDeclarations> (ResourcesJSON);

			ValidateJson ();
		}

		void ValidateJson ()
		{
			if (Buildings.Buildings == null)
				throw new InvalidOperationException ($"JSON failed validation, No buildings?");

			foreach (var b in Buildings.Buildings)
			{
				if (b.ValidRegions == null)
					throw new InvalidOperationException ($"JSON failed validation, {b.Name} has no valid regions?");

				foreach (var yield in b.Yield.AsNotNull ())
					ValidateResource (yield.Name);

				foreach (var convertYield in b.ConversionYield.AsNotNull ())
				{
					foreach (var provide in convertYield.Provides.AsNotNull ())
						ValidateResource (provide.Name);
					foreach (var cost in convertYield.Cost.AsNotNull ())
						ValidateResource (cost.Name);
				}

				foreach (var region in b.ValidRegions)
					ValidateRegion (region);
			}
		}

		void ValidateResource (string name)
		{
			if (!Resources.Resources.Any (x => x.Name == name))
				throw new InvalidOperationException ($"JSON failed validation, unable to find resource - {name}");
		}

		void ValidateRegion (string name)
		{
			if (!Regions.Regions.Any (x => x.Name == name))
				throw new InvalidOperationException ($"JSON failed validation, unable to find region - {name}");
		}
	}
}
