.PHONY: all clean release debug_cpu doc

# For now, default to building with DEBUG
all: 
	@clear
	@cargo build --features "debug_cpu debug_ppu" 

debug_cpu: 
	@clear
	@cargo build --features "debug_cpu"

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
