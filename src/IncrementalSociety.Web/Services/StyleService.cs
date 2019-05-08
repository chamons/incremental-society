using System.Collections.Generic;
using System.Linq;

using IncrementalSociety.Json;

namespace IncrementalSociety.Web.Services
{
	class StyleService
	{
		GameService GameService;

		public StyleService (GameService gameService)
		{
			GameService = gameService;
		}

		// STUB_DATA - Filter by age
		public IEnumerable<ResourceDeclaration> Resources => GameService.Loader.Resources.Resources;
		
		public string GetImageFilename (string name)
		{
			return GetImageFilename (Resources.First (x => x.Name == name));
		}

		public string GetImageFilename (ResourceDeclaration decl)
		{
			string name = decl.Name.ToLower ().Replace (' ', '-');
			if (decl.ImageHasAgePrefix)
				return $"images\\{GameService.State.Age}-{name}.png";
			else
				return $"images\\{name}.png";
		}

		public string GetResourceDeltaClass (double count)
		{
			if (count < -.001) {
				return "red";
			}
			if (count > .001) {
				return "green";
			}
			return "clear";
		}	
	}
}
