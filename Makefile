Q=$(if $(V),,@)

run::
	$(Q) dotnet run -nologo --project src/IncrementalSociety.Web/IncrementalSociety.Web.csproj

build::
	$(Q) dotnet build -nologo

clean::
	$(Q) dotnet clean -nologo

test::
	$(Q) dotnet test -nologo
