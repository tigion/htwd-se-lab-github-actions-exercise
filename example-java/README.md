# Example Java

A small Java learning project demonstrating object-oriented programming and
unit testing with `?`.

> [!WARNING]
> Two tests intentionally fail. The errors are part of the exercise and should
> be corrected during the task.

<!--toc:start-->

- [Project Structure](#project-structure)
- [Running the Project](#running-the-project)
  - [Run](#run)
  - [Test](#test)
- [Testing Guidelines](#testing-guidelines)
  - [Unit Test Naming Convention](#unit-test-naming-convention)
  - [AAA Pattern (Arrange - Act - Assert)](#aaa-pattern-arrange-act-assert)
- [Java Basics](#java-basics)
  - [`trim()` and `strip()`](#trim-and-strip)

<!--toc:end-->

## Project Structure

```txt
example-java
├── pom.xml
├── src
│   ├── main
│   │   └── java
│   │       └── example
│   │           ├── Main.java
│   │           └── Town.java
│   └── test
│       └── java
│           └── example
│               ├── TestMain.java
│               └── TestTown.java
├── .gitignore
└── README.md
```

| Item            | Description                                                                                                                   |
| --------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `pom.xml`       | Defines the Maven project configuration, dependencies, and build settings.                                                    |
| `src/`          | Contains the source code and unit tests of the application.                                                                   |
| `main/java/`    | Contains the application source code.                                                                                         |
| `example/`      | Contains the Java package with the application code and tests.                                                                |
| `Main.java`     | Contains the `main()` method used to run the application.                                                                     |
| `Town.java`     | Contains the `Town` class representing a town with a name (`name`) and number of residents (`residents`) and related methods. |
| `test/java/`    | Contains the unit tests for the application source code.                                                                      |
| `TestMain.java` | Contains unit tests for the `Main` class.                                                                                     |
| `TestTown.java` | Contains unit tests for the `Town` class.                                                                                     |
| `.gitignore`    | Specifies files and directories that should not be tracked by Git.                                                            |
| `README.md`     | Contains project documentation and instructions.                                                                              |

## Running the Project

### Run

Build the project and create the JAR file:

```sh
mvn package
```

If you want to build the project without running tests:

```sh
mvn package -DskipTests
```

Start the application with `java -cp target/example-1.0-SNAPSHOT.jar example.Main`:

```console
$ java -cp target/example-1.0-SNAPSHOT.jar example.Main
Dresden has 556780 residents.
Town('Dresden', 556780)
Dresden
556780
```

### Test

Check the Maven version:

```sh
mvn --version
```

Remove previously generated build artifacts:

```sh
mvn clean
```

Run all unit tests:

```sh
# Run all tests
mvn test
```

Run specific tests (classes or methods):

```sh
# Run a specific test class
mvn -Dtest=TestTown test

# Run multiple test classes
mvn -Dtest=TestTown,TestCity test

# Run a specific test method
mvn -Dtest=TestTown#testTownValues test

# Run test classes matching a pattern
mvn -Dtest=*TestTown test
```

## Testing Guidelines

### Unit Test Naming Convention

A common naming pattern for unit tests is:

```txt
test_<class><feature or method><expected behavior>
```

Example:

```txt
test_town_name_setter_sets_and_gets_correct_value
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

```java
// Arrange
String name = "Dresden";
int residents = 556780;

// Act
Town town = new Town(name, residents);

// Assert
assertEquals(name, town.getName());
assertEquals(residents, town.getResidents());
```

## Java Basics

### `trim()` and `strip()`

Both methods remove leading and trailing whitespace from a `String`, but
`strip()` is the preferred method for modern Java applications.

- `trim()` removes ASCII whitespace (characters <= `U+0020`) like spaces, tabs,
  and newlines.

- `strip()` removes a broader set of Unicode whitespace characters (available
  since Java 11).
