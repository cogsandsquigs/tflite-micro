# The setup command to set up your workspace environment.
# This command should be run once when you first clone the repository.
setup:
	@echo "Setting up your workspace..."
	@echo "This requires the Rust toolchain to be installed!"
	@echo "Installing cargo requirements..."
	@cargo install bindgen
	@echo "Done!"

# This cleans the project's build artefacts.
clean:
	@echo "Cleaning project..."
	@cargo clean