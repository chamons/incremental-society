using System.Linq;
using IncrementalSociety.Json;

namespace IncrementalSociety
{
	public class EdictsEngine
	{
		JsonLoader Json;

		public EdictCooldownConfig EdictConfig;

		public EdictsEngine (JsonLoader json)
		{
			Json = json;
			EdictConfig = new EdictCooldownConfig (json.Edicts.Edicts.Select (x => x.Name));
		}
	}
}