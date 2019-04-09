using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace IncrementalSociety.Web.Services
{
    public class GameService
    {
        JsonLoader Loader;
        public GameService ()
        {
            Loader = JsonLoader.Load ();
        }

        public string[] ResourcesNames => Loader.Resources.Resources.Select (x => x.Name).ToArray ();
    }
}
