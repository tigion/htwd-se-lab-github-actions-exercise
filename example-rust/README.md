# Example Rust

A small Rust learning project demonstrating struct-based programming and
unit testing with the built-in `#[test]` framework.

> [!WARNING]
> Two tests intentionally fail. The errors are part of the exercise and should
> be corrected during the task.

<!--toc:start-->

- [Project Structure](#project-structure)
- [Requirements](#requirements)
- [Running the Project](#running-the-project)
  - [Run](#run)
  - [Test](#test)
- [Testing Guidelines](#testing-guidelines)
  - [Unit Test Naming Convention](#unit-test-naming-convention)
  - [AAA Pattern (Arrange - Act - Assert)](#aaa-pattern-arrange-act-assert)
- [Rust Basics](#rust-basics)
  - [Error Handling with `Result`](#error-handling-with-result)

<!--toc:end-->

## Project Structure

```txt
example-rust
├── Cargo.toml
├── src
│   ├── main.rs
│   └── town.rs
├── .gitignore
└── README.md
```

| Item         | Description                                                            |
| ------------ | ---------------------------------------------------------------------- |
| `src/`       | Contains the source code of the application.                           |
| `main.rs`    | Declares the `town` module and contains the `main()` function.         |
| `town.rs`    | Contains the `Town` struct and the unit tests.                         |
| `.gitignore` | Specifies files and directories that should not be tracked by Git.     |
| `Cargo.lock` | Pins the exact versions of dependencies to ensure reproducible builds. |
| `Cargo.toml` | Defines the Cargo project configuration and build settings.            |
| `README.md`  | Contains project documentation and instructions.                       |

## Requirements

- Rust **1.85+** — Compiles and runs the application and tests (includes Cargo).

Verify your installed versions:

```sh
rustc --version
cargo --version
```

## Running the Project

### Run

Build and start the application:

```sh
cargo run
```

Expected output:

```console
$ cargo run
Dresden has 556780 residents.
Town('Dresden', 556780)
Dresden
556780
Town('DRESDEN', 556781)
```

### Test

Run all unit tests:

```sh
cargo test
```

Run specific tests (names or substrings):

```sh
# Run a specific test
cargo test set_name_rejects_invalid_name

# Run tests matching a name or substring
cargo test set_name
```

## Testing Guidelines

The unit tests live directly in the source files within a `#[cfg(test)] mod tests`
block, e.g. in `src/town.rs`. The `#[cfg(test)]` attribute ensures the test code
is only compiled and run by `cargo test`, not into the binary build by
`cargo build`.

### Unit Test Naming Convention

Rust tests are marked with the `#[test]` attribute. Test functions can use
descriptive `snake_case` names following a pattern such as:

```txt
<feature or method><expected behavior>
```

The expected behavior usually starts with a verb describing the tested behavior,
such as `updates`, `rejects`, or `returns`.

Example:

```txt
set_name_rejects_invalid_name
│        │
│        └── expected behavior
└── feature or method being tested
```

### AAA Pattern (Arrange - Act - Assert)

The AAA pattern is a common way to structure unit tests:

- **Arrange**: prepare test data and objects.
- **Act**: execute the code that is being tested.
- **Assert**: verify that the result is as expected.

```rust
// Arrange
let name = "Dresden";
let residents = 556780;

// Act
let town = Town::new(name, residents).expect("valid town values");

// Assert
assert_eq!(name, town.name());
assert_eq!(residents, town.residents());
```

## Rust Basics

TODO
