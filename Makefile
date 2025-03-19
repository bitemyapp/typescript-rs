# fixtures/sample.js: fixtures/sample.ts
# 	rm -f fixtures/sample.js
# 	cargo run -- fixtures/sample.ts
# 	node fixtures/sample.js

rs-test-sample:
	rm -f fixtures/sample.js
	cargo run -- fixtures/sample.ts
	node fixtures/sample.js

rs-test-csg:
	rm -f fixtures/csg.js
	cargo run -- fixtures/csg.ts
	node fixtures/csg.js


tsc-test-sample:
	rm -f fixtures/sample.js
	./typescript-go/_submodules/TypeScript/node_modules/typescript/bin/tsc fixtures/sample.ts
	node fixtures/sample.js

tsc-test-csg:
	rm -f fixtures/csg.js
	./typescript-go/_submodules/TypeScript/node_modules/typescript/bin/tsc fixtures/csg.ts
	node fixtures/csg.js
