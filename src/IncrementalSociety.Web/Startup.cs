using IncrementalSociety.Web.Services;
using Microsoft.AspNetCore.Components.Builder;
using Microsoft.Extensions.DependencyInjection;

namespace IncrementalSociety.Web
{
	public class Startup
	{
		public void ConfigureServices (IServiceCollection services)
		{
			services.AddSingleton<ActionService> ();
			services.AddSingleton<BuildingService> ();
			services.AddSingleton<GameService> ();
			services.AddSingleton<StyleService> ();
		}

		public void Configure (IComponentsApplicationBuilder app)
		{
			app.AddComponent<App> ("app");
		}
	}
}
