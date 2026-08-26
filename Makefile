RUST_TARGET := wasm32-unknown-unknown
SOURCE_MANIFESTS := $(filter-out sources/test/Cargo.toml,$(wildcard sources/*/Cargo.toml))
SOURCE_DIRS := $(patsubst %/Cargo.toml,%,$(SOURCE_MANIFESTS))
SOURCE_NAMES := $(notdir $(SOURCE_DIRS))
RELEASE_DIR := release
PACKAGES := $(addprefix $(RELEASE_DIR)/,$(addsuffix .aix,$(SOURCE_NAMES)))

.PHONY: all setup check package clean rebuild FORCE

all: check package

setup:
	@command -v cargo >/dev/null 2>&1 || { echo "error: Cargo is not installed in WSL" >&2; exit 1; }
	@command -v rustup >/dev/null 2>&1 || { echo "error: rustup is not installed in WSL" >&2; exit 1; }
	@rustup target list --installed | grep -qx "$(RUST_TARGET)" || rustup target add $(RUST_TARGET)
	@command -v aidoku >/dev/null 2>&1 || cargo install --git https://github.com/Aidoku/aidoku-rs aidoku-cli

check: setup
	@set -e; for src in $(SOURCE_DIRS); do \
		echo "Checking $$src"; \
		(cd "$$src" && cargo fmt --all --check && cargo clippy -- -D warnings); \
	done

package: $(PACKAGES)
	@echo "Generated packages:"
	@printf '  %s\n' $(PACKAGES)

$(RELEASE_DIR)/%.aix: FORCE setup
	@echo "Packaging sources/$*"
	@mkdir -p "$(RELEASE_DIR)"
	@cd "sources/$*" && aidoku package
	@mv "sources/$*/package.aix" "$@"
	@test -f "$@" || { echo "error: Aidoku CLI did not create $@" >&2; exit 1; }

FORCE:

rebuild: clean package

clean:
	@set -e; for src in $(SOURCE_DIRS); do \
		(cd "$$src" && cargo clean && rm -f package.aix); \
	done
	@rm -f "$(RELEASE_DIR)"/*.aix
	@rmdir "$(RELEASE_DIR)" 2>/dev/null || true
