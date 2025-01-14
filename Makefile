# Makefile for Rust project

# Variables
CARGO := cargo
PROJECT_NAME := lib-test-ci

# Default target
all: build

# Build the project in development mode
build-dev:
	$(CARGO) build

# Build the project in release mode
build-release:
	$(CARGO) build --release

# Run tests
test:
	$(CARGO) test

# Clean the project
clean:
	$(CARGO) clean

# Run the project
run:
	$(CARGO) run

# Format the code
fmt:
	$(CARGO) fmt

# Check for warnings and errors
check:
	$(CARGO) check



.PHONY: build-dev build-release