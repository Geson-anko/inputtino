# Contributing to inputtino-python

## Development Environment Setup

### Prerequisites

- Python 3.10 or higher
- Docker (recommended)
- Make
- Git

### Setting Up

1. Fork and clone the repository:

```bash
git clone https://github.com/your-username/inputtino-python.git
cd inputtino-python
```

2. Set up development environment:

Using Docker (recommended):

```bash
# Build development container
make docker-build

# Start container
make docker-up

# Attach to development shell
make docker-attach
```

## Development Workflow

### Code Quality Tools

- Ruff: Code formatting and linting
- Pyright: Static type checking
- Pytest with pytest-mock: Testing

### Common Commands

```bash
# Run all checks
make run

# Individual checks
make format        # Run formatters
make test         # Run unit tests
make test-full    # Run all tests including practical tests
make type         # Run type checker
make clean        # Clean generated files
```

## Coding Standards

### Python Code

- Use type hints for all function arguments and return values
- Prefer built-in `list` and `tuple` over `typing.List` and `typing.Tuple`
- All public classes and methods must have docstrings (Google Style)
- Follow PEP 8 guidelines
- Keep functions focused and single-purpose
- Prioritize code readability

Example docstring:

```python
def function_name(param1: str, param2: list[int]) -> bool:
    """Short description of function.

    Args:
        param1: Description of first parameter
        param2: Description of second parameter

    Returns:
        Description of return value

    Raises:
        ValueError: Description of when this is raised
    """
    pass
```

### C++ Code

- Use pybind11 for Python bindings
- Run `make includes` to see the include directories path for your IDE setup
- After modifying C++ binding code, always run `make stubs` to regenerate Python stubs
- Follow modern C++ practices
- Keep binding code simple and focused on Python integration
- Document any platform-specific considerations

## Pull Request Process

1. Create feature branch:

```bash
git checkout -b feature/name
```

2. Make changes following coding standards

3. Run all checks:

```bash
make run
```

4. Commit with clear message:

```bash
git commit -m "Add feature: description"
```

5. Push to fork and submit pull request

## Testing

- Write tests for all new features
- Run all tests locally before pull requests
- Mark practical tests with `@pytest.mark.practical`
- Maintain test coverage
- Use mocks appropriately for external dependencies

## Questions

- Open issue for bugs/features
- Use discussions for questions

## Code of Conduct

Please be respectful and professional in all interactions.
