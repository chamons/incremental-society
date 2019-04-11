using System.Collections.Generic;
using System.Linq;

using IncrementalSociety.Model;
using IncrementalSociety.Resources;


namespace IncrementalSociety.Web.Services
{
    public class GameService
    {
        JsonLoader Loader;
        GameState State;

        public GameService ()
        {
            Loader = JsonLoader.Load ();
            State = GameEngine.CreateNewGame ();
        }

        // STUB_DATA - Filter by age
        public IEnumerable<ResourceDeclaration> Resources => Loader.Resources.Resources;

        public IEnumerable<GameAction> Actions => Loader.Actions.Actions;

        public IEnumerable<Region> Regions => State.Regions;

        public int GetResourceCount (string name)
        {
            var resource = State.Resources.FirstOrDefault (x => x.Name == name);
            return resource != null ? resource.Amount : 0;
        }

        public string GetImageFilename (ResourceDeclaration decl)
        {
            string name = decl.Name.ToLower ().Replace (' ', '-');
            if (decl.Image_has_age_prefix)
                return $"images\\{State.Age}-{name}.png";
            else
                return $"images\\{name}.png";
        }
    }
}
