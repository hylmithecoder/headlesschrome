.PHONY: all run build check test clean help

all: check build

# Run the screenshot & video recorder engine
run:
	rustup run stable cargo run

# Build the release binary
build:
	rustup run stable cargo build --release

# Check project compilation
check:
	rustup run stable cargo check

# Run tests
test:
	rustup run stable cargo test

# Clean build artifacts
clean:
	rustup run stable cargo clean

# Display available commands
help:
	@echo "Available Makefile commands:"
	@echo "  make run    - Run the screenshot & video recorder engine"
	@echo "  make build  - Build optimized release binary"
	@echo "  make check  - Run cargo check for errors/warnings"
	@echo "  make test   - Run test suite"
	@echo "  make clean  - Clean cargo target build directory"
