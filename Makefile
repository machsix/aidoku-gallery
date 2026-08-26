RUST_TARGET := wasm32-unknown-unknown

.PHONY: all setup check package clean rebuild

all: package

setup:
	@command -v cargo >/dev/null 2>&1 || { echo "error: Cargo is not installed in WSL" >&2; exit 1; }
	@command -v rustup >/dev/null 2>&1 || { echo "error: rustup is not installed in WSL" >&2; exit 1; }
	@rustup target list --installed | grep -qx "$(RUST_TARGET)" || rustup target add $(RUST_TARGET)
	@command -v aidoku >/dev/null 2>&1 || cargo install --git https://github.com/Aidoku/aidoku-rs aidoku-cli

check: setup
	@set -e; for src in sources/*; do \
		echo "Checking $$src"; \
		(cd "$$src" && cargo fmt --all --check && cargo clippy -- -D warnings); \
	done

package: check
	@set -e; for src in sources/*; do \
		echo "Packaging $$src"; \
		(cd "$$src" && aidoku package && test -f package.aix); \
	done

rebuild: clean package

clean:
	@set -e; for src in sources/*; do \
		(cd "$$src" && cargo clean && rm -f package.aix); \
	done
