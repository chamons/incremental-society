all::
	cd src/lib/ && wasm-pack build --dev
	cd src/lib/pkg && npm link
	cd src/ui/ && npm link incremental-society

run:: all
	cd src/ui/ && npm run start

setup::
	cd src/ui/ && npm install

clean::
	rm -rf src/lib/pkg/
	rm -rf src/lib/target/
