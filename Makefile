.PHONY: all clean release

all: 
	@clear
	@cargo build

clean:
	@cargo clean

release:
	@cargo build --release

