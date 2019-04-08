using System;
using System.IO;
using System.Reflection;

namespace IncrementalSociety
{
    public class Resources
    {
        public string ActionsJSON { get; }
        public string BuildingsJSON { get; }
        public string GameJSON { get; }
        public string RegionsJSON { get; }
        public string ResourcesJSON { get; }

        Resources (string actions, string buildings, string game, string regions, string resources)
        {
            ActionsJSON = actions;
            BuildingsJSON = buildings;
            GameJSON = game;
            RegionsJSON = regions;
            ResourcesJSON = resources;
        }

        static string ReadResource (string filename)
        {
            var x = Assembly.GetExecutingAssembly ().GetManifestResourceNames ();
            using (Stream stream = Assembly.GetExecutingAssembly ().GetManifestResourceStream ("IncrementalSociety.data." + filename))
                using (StreamReader reader = new StreamReader (stream))
                    return reader.ReadToEnd ();
        }

        public static Resources Load ()
        {
            return new Resources (ReadResource ("actions.json"), ReadResource ("buildings.json"), ReadResource ("game.json"), ReadResource ("regions.json"), ReadResource ("resources.json"));
        }
    }
}
