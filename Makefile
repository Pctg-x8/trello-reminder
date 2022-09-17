CARGO_TARGET_DIR = target/x86_64-unknown-linux-musl/release

.PHONY: all

all:
	$(MAKE) package.zip

$(CARGO_TARGET_DIR)/bootstrap:
	cargo build --release --target x86_64-unknown-linux-musl

package.zip: $(CARGO_TARGET_DIR)/bootstrap
	cd $(CARGO_TARGET_DIR) && zip ../../../package.zip bootstrap
