# vet
vet-check: # Verify code syntax [synonym: check]
	@cargo check --future-incompat-report --all-features --verbose
.PHONY: vet-check

check: vet-check
.PHONY: check

vet-format: # Check formats without changes [synonym: format, fmt]
	@cargo fmt --all --check
.PHONY: vet-format

format: vet-format
.PHONY: format

fmt: vet-format
.PHONY: fmt

vet-lint: # Apply lint checks [synonym: lint]
	@cargo clippy --all-features
.PHONY: vet-lint

lint: vet-lint
.PHONY: lint

vet-all: vet-check vet-format vet-lint # Check all vet targets [synonym: vet]
.PHONY: vet-all

vet: vet-all
.PHONY: vet

# test
test-integration: # Run integration tests
	@cargo test --test integration --all-features
.PHONY: test-integration

test-unit: # Run unit tests
	@cargo test --lib --all-features
.PHONY: test-unit

test-all: test-unit test-integration # Run all tests [synonym: test]
.PHONY: test-all

test: test-all
.PHONY: test

# build
build-debug: # Build in debug mode [synonym: build]
	cargo build
.PHONY: build-debug

build: build-debug
.PHONY: build
# }}}

# utility
clean: # Clean up
	@cargo clean
.PHONY: clean

package: # Create the crate package
	@cargo package
.PHONY: package

publish: # Publish the crate package
	@cargo publish
.PHONY: publish

help: # Display this message
	@set -uo pipefail; \
	grep --extended-regexp '^[-_0-9a-z\%\:\\ ]+: ' \
		$(firstword $(MAKEFILE_LIST)) | \
		grep --extended-regexp ' # ' | \
		sed --expression='s/\( [-_0-9a-z\%\:\\ ]*\) #/ #/' | \
		tr --delete \\\\ | \
		awk 'BEGIN {FS = ": # "}; \
			{printf "\033[38;05;222m%-17s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help

.DEFAULT_GOAL := test
