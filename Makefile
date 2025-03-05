all: build

.PHONY: build
build:
	@cargo build --release

.PHONY: install
install:
	@cp ./target/release/kklogIced /usr/local/bin/kklogIced

.PHONY: package
package:
	@cp /usr/local/bin/kklog ./.vscode/kklog/kklog
	@cp /usr/local/bin/kklogIced ./.vscode/kklog/kklogIced
	@cp ~/.kkconf.yaml ./.vscode/kklog/kkconf.yaml
