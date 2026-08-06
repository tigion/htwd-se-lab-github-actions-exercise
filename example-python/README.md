# Example Python

A small Python learning project demonstrating object-oriented programming and
unit testing with `unittest`.

> [!WARNING]
> Two tests intentionally fail. The errors are part of the exercise and should
> be corrected during the task.

<!--toc:start-->

- [Project Structure](#project-structure)
- [Running the Project](#running-the-project)
  - [Setup](#setup)
  - [Run](#run)
  - [Test](#test)
- [Testing Guidelines](#testing-guidelines)
  - [Unit Test Naming Convention](#unit-test-naming-convention)
  - [AAA Pattern (Arrange - Act - Assert)](#aaa-pattern-arrange-act-assert)
- [Python Basics](#python-basics)
  - [Placeholder Syntax](#placeholder-syntax)

<!--toc:end-->

## Project Structure

```txt
example-python
├── src
│   └── town_example
│       ├── __init__.py
│       ├── main.py
│       └── town.py
├── tests
│   └── test_town.py
├── .gitignore
├── pyproject.toml
└── README.md
```

| Item             | Description                                                                                                                   |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `src/`           | Contains the source code of the application.                                                                                  |
| `town_example/`  | Contains the Python package with the application code.                                                                        |
| `__init__.py`    | Marks the directory as a Python package.                                                                                      |
| `main.py`        | Contains the `main()` function used to run the application.                                                                   |
| `town.py`        | Contains the `Town` class representing a town with a name (`name`) and number of residents (`residents`) and related methods. |
| `tests/`         | Contains the unit tests for the application code.                                                                             |
| `test_town.py`   | Contains the `TestTown` class with unit tests for testing the `Town` class.                                                   |
| `pyproject.toml` | Defines project metadata, package configuration, and build settings.                                                          |
| `.gitignore`     | Specifies files and directories that should not be tracked by Git.                                                            |
| `README.md`      | Contains project documentation and instructions.                                                                              |

## Running the Project

### Setup

Install the project in editable mode:

```sh
python3 -m pip install -e .
```

### Run

Start the application with `python3 -m town_example.main`:

```console
$ python3 -m town_example.main
Dresden has 556780 residents.
Town('Dresden', 556780)
Dresden
556780
```

### Test

- Display Python version:

  ```sh
  python3 --version
  ```

- Run automatically discovered tests:

  ```sh
  # Discover tests in a specific directory
  python3 -m unittest discover -s tests

  # Discover tests in a specific directory with a specific filename pattern
  python3 -m unittest discover -s tests -p "test_town_*.py"
  ```

- Run specific tests (modules, classes, methods):

  ```sh
  # Run all tests in a test module
  python3 -m unittest tests.test_town

  # Run all test methods in a specific test class
  python3 -m unittest tests.test_town.TestTown

  # Run one specific test method
  python3 -m unittest tests.test_town.TestTown.test_town_name_setter_rejects_invalid_name
  ```

## Testing Guidelines

### Unit Test Naming Convention

A common naming pattern for unit tests is:

```txt
test_<class><feature or method><expected behavior>
```

Example:

```txt
test_town_name_setter_rejects_invalid_name
│    │    │           │
│    │    │           └── expected behavior
│    │    └── feature or method being tested
│    └── class under test
└── required prefix for unittest test discovery
```

The `test_` prefix is required by Python's `unittest` framework to
discover and execute test methods.

### AAA Pattern (Arrange - Act - Assert)

The AAA pattern is a common way to structure unit tests:

- **Arrange**: prepare test data and objects.
- **Act**: execute the code that is being tested.
- **Assert**: verify that the result is as expected.

```python
# Arrange
name = "Dresden"
residents = 556780

# Act
town = Town(name, residents)

# Assert
self.assertEqual(town.name, name)
self.assertEqual(town.residents, residents)
```

## Python Basics

### Placeholder Syntax

Python provides two ways to leave code empty or incomplete:

- `pass` is used when no action is required but Python needs a statement.
- `...` (Ellipsis) is used as a placeholder for code that is not implemented yet.
