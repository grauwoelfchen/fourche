# verify {{{
verify\:check:
	@cargo check --all --verbose
.PHONY: verify\:check

check: | verify\:check
.PHONY: check

verify\:format:
	@cargo fmt --all -- --check
.PHONY: verify\:format

format: | verify\:format
.PHONY: format

fmt: | verify\:format
.PHONY: fmt

verify\:lint:
	@cargo clippy --all-targets
.PHONY: verify\:lint

lint: | verify\:lint
.PHONY: lint
# }}}

# test {{{
test\:all:
	@cargo test --lib
.PHONY: test\:all

test: | test\:all
.PHONY: test
# }}}

# build {{{
build\:debug:
	cargo build
.PHONY: build\:debug

build: | build\:debug
.PHONY: build
# }}}

# utilities {{{
clean:
	@cargo clean
.PHONY: clean
# }}}

.DEFAULT_GOAL = test:all
default: verify\:check verify\:format verify\:lint test\:all
