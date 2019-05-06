Q=$(if $(V),,@)

build::
	$(Q) dotnet build -nologo --no-restore

run::
	$(Q) dotnet run -nologo --project src/IncrementalSociety.Web/IncrementalSociety.Web.csproj

clean::
	$(Q) dotnet clean -nologo

test::
	$(Q) dotnet test -nologo

sass::
	$(Q) sass --watch src/IncrementalSociety.Web/wwwroot/css/site.sass src/IncrementalSociety.Web/wwwroot/css

records::
	$(Q) dotnet records src/lib/Model/GameState.cs -o src/lib/Model/
