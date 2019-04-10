using IncrementalSociety.Resources;
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

        public ResourceDeclaration[] Resourcs => Loader.Resources.Resources.ToArray ();

        public string GetImageFilename (ResourceDeclaration decl)
        {
            string name = decl.Name.ToLower ().Replace (' ', '-');
            if (decl.Image_has_age_prefix)
                return $"images\\stone-{name}.png"; // STUB_DATA
            else
                return $"images\\{name}.png";
        }
    }
}
