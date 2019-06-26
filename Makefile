# verify {{{
verify\:check:  ## Verify code syntax [alias: check]
	@cargo check --all --verbose
.PHONY: verify\:check

check: | verify\:check
.PHONY: check

verify\:format:  ## Verify format without changes [alias: verify:fmt, format, fmt]
	@cargo fmt --all -- --check
.PHONY: verify\:format

format: | verify\:format
.PHONY: format

fmt: | verify\:format
.PHONY: fmt

verify\:lint:  ## Verify coding style using clippy [alias: lint]
	@cargo clippy --all-targets
.PHONY: verify\:lint

lint: | verify\:lint
.PHONY: lint
# }}}

# test {{{
test\:all:  ## Run all unit tests [alias: test]
	@cargo test --lib
.PHONY: test\:all

test: | test\:all
.PHONY: test
# }}}

# build {{{
build\:debug:  ## Build in debug mode [alias: build]
	cargo build
.PHONY: build\:debug

build: | build\:debug
.PHONY: build
# }}}

# utilities {{{
clean:  ## Clean up
	@cargo clean
.PHONY: clean

help:  ## Display this message
	@grep -E '^[0-9a-z\:\\]+: ' $(MAKEFILE_LIST) | \
	  grep -E '  ## ' | \
	  sed -e 's/\(\s|\(\s[0-9a-z\:\\]*\)*\)  /  /' | \
	  tr -d \\\\ | \
	  awk 'BEGIN {FS = ":  ## "};  \
	       {printf "\033[38;05;222m%-14s\033[0m %s\n", $$1, $$2}' | \
	  sort
.PHONY: help
# }}}

.DEFAULT_GOAL = test:all
default: verify\:check verify\:format verify\:lint test\:all
