.PHONY: all clean release debug

# For now, default to building with DEBUG
all: 
	@clear
	@cargo build --features "DEBUG"

debug: 
	@clear
	@cargo build --features "DEBUG"

clean:
	@cargo clean

release:
	@cargo build --release

