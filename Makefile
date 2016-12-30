.PHONY: all clean release debug doc

# For now, default to building with DEBUG
all: 
	@clear
	@cargo build --features "debug"

debug: 
	@clear
	@cargo build --features "debug"

doc: 
	cargo rustdoc -- \
		--no-defaults \
		--passes strip-hidden \
		--passes collapse-docs \
		--passes unindent-comments \
		--passes strip-priv-imports

clean:
	@cargo clean

release:
	@cargo build --release
