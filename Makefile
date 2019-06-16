Q=$(if $(V),,@)

build::
	$(Q) dotnet build -nologo --no-restore

run::
	$(Q) dotnet run -nologo --project src/IncrementalSociety.Web/IncrementalSociety.Web.csproj

clean::
	$(Q) dotnet clean -nologo

test::
	$(Q) dotnet test -nologo

test-fast::
	$(Q) dotnet test -nologo

sass::
	$(Q) sass --watch src/IncrementalSociety.Web/wwwroot/css/site.sass src/IncrementalSociety.Web/wwwroot/css

records::
	$(Q) dotnet records src/lib/Model/GameState.csx -o src/lib/Model/

blog::
	$(Q) cd docs && bundle exec jekyll serve
