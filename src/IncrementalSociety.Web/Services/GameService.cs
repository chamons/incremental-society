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

        // STUB_DATA - Filter by age
        public IEnumerable<ResourceDeclaration> Resources => Loader.Resources.Resources;
        public IEnumerable<GameAction> Actions => Loader.Actions.Actions;

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
