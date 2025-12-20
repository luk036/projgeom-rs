.PHONY: install install-dev test test-cov lint format type-check clean build

# Install the package
install:
	pip install -e .

# Install the package with development dependencies
install-dev:
	pip install -e ".[dev]"

# Run tests
test:
	pytest

# Run tests with coverage
test-cov:
	pytest --cov=projgeom --cov-report=html --cov-report=term

# Run linting
lint:
	ruff check src/projgeom tests

# Format code
format:
	ruff format src/projgeom tests
	black src/projgeom tests

# Run type checking
type-check:
	mypy src/projgeom

# Clean build artifacts
clean:
	rm -rf build/
	rm -rf dist/
	rm -rf *.egg-info/
	rm -rf .pytest_cache/
	rm -rf .coverage
	rm -rf htmlcov/
	find . -type d -name __pycache__ -exec rm -rf {} +
	find . -type f -name "*.pyc" -delete

# Build the package
build: clean
	python -m build

# Run all checks
check: lint type-check test

# Development setup
setup: install-dev
	pre-commit install