using IncrementalSociety.Json;
using Newtonsoft.Json;
using System;
using System.IO;
using System.Reflection;

namespace IncrementalSociety.Json
{
    public class JsonLoader
    {
        public string ActionsJSON { get; }
        public ActionDeclarations Actions { get; }

        public string BuildingsJSON { get; }
        public BuildingDeclarations Buildings { get; }

        public string GameJSON { get; }
        public GameDeclarations Game { get; }

        public string RegionsJSON { get; }
        public RegionDeclarations Regions { get; }

        public string ResourcesJSON { get; }
        public ResourceDeclarations Resources { get; }

        JsonLoader (string actions, string buildings, string game, string regions, string resources)
        {
            ActionsJSON = actions;
            Actions = JsonConvert.DeserializeObject<ActionDeclarations> (ActionsJSON);

            BuildingsJSON = buildings;
            Buildings = JsonConvert.DeserializeObject<BuildingDeclarations> (BuildingsJSON);

            GameJSON = game;
            Game = JsonConvert.DeserializeObject<GameDeclarations> (GameJSON);

            RegionsJSON = regions;
            Regions = JsonConvert.DeserializeObject<RegionDeclarations> (RegionsJSON);

            ResourcesJSON = resources;
            Resources = JsonConvert.DeserializeObject<ResourceDeclarations> (ResourcesJSON);
        }


        static string ReadJSONText (string filename)
        {
            var x = Assembly.GetExecutingAssembly ().GetManifestResourceNames ();
            using (Stream stream = Assembly.GetExecutingAssembly ().GetManifestResourceStream ("IncrementalSociety.data." + filename))
                using (StreamReader reader = new StreamReader (stream))
                    return reader.ReadToEnd ();
        }

        public static JsonLoader Load ()
        {
            return new JsonLoader (ReadJSONText ("actions.json"), ReadJSONText ("buildings.json"), ReadJSONText ("game.json"), ReadJSONText ("regions.json"), ReadJSONText ("resources.json"));
        }
    }
}
