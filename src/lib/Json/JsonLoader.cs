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

		public string AreaJSON { get; }
		public AreaDeclarations Areas { get; }

		public string ResourcesJSON { get; }
		public ResourceDeclarations Resources { get; }

		public string ResearchJSON { get; }
		public ResearchDeclarations Research { get; }

		public string EdictsJSON { get; }
		public EdictsDeclarations Edicts { get; }

		public string RegionNameJSON { get; }

		public JsonLoader (string buildings, string game, string areas, string resources, string research, string edicts, string regionNames, bool validate = true)
		{
			BuildingsJSON = buildings;
			Buildings = JsonConvert.DeserializeObject<BuildingDeclarations> (BuildingsJSON);

			GameJSON = game;
			Game = JsonConvert.DeserializeObject<GameDeclarations> (GameJSON);

			AreaJSON = areas;
			Areas = JsonConvert.DeserializeObject<AreaDeclarations> (AreaJSON);

			ResourcesJSON = resources;
			Resources = JsonConvert.DeserializeObject<ResourceDeclarations> (ResourcesJSON);

			ResearchJSON = research;
			Research = JsonConvert.DeserializeObject<ResearchDeclarations> (ResearchJSON);

			EdictsJSON = edicts;
			Edicts = JsonConvert.DeserializeObject<EdictsDeclarations> (EdictsJSON);

			RegionNameJSON = regionNames;

			if (validate)
				ValidateJson ();
		}

		void ValidateJson ()
		{
			if (Buildings.Buildings == null)
				throw new InvalidOperationException ($"JSON failed validation, No buildings?");

			foreach (var b in Buildings.Buildings)
			{
				if (b.ValidAreas == null)
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

				foreach (var region in b.ValidAreas)
					ValidateArea (region);
			}

			foreach (var climate in Areas.Climates.AsNotNull ())
				if (!climate.AreaChances.Select (x => x.Chance).Sum ().Is (1.0))
					throw new InvalidOperationException ($"JSON failed validation, chances in area {climate.Name	} did not add to 1.0");

			if (Areas.Climates == null)
				throw new InvalidOperationException ($"JSON failed validation, found no climates");

			if (Areas.Features == null)
				throw new InvalidOperationException ($"JSON failed validation, found no features");
		}

		void ValidateResource (string name)
		{
			if (!Resources.Resources.Any (x => x.Name == name))
				throw new InvalidOperationException ($"JSON failed validation, unable to find resource - '{name}'");
		}

		void ValidateArea (string name)
		{
			if (name == "Any")
				return;
			if (!Areas.Areas.Any (x => x.Name == name))
				throw new InvalidOperationException ($"JSON failed validation, unable to find area - '{name}'");
		}
	}
}
