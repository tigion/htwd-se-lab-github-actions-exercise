# HTWD | Übung: GitHub Actions mit Unit-Tests

Das Repository enthält Übungsprojekte für die Praktikumsaufgabe zu [GitHub
Actions][08-github-actions] im Fach Software Engineering II.

[08-github-actions]: https://www.informatik.htw-dresden.de/~zirkelba/praktika/se/git-docs-collab/08-github-actions.html#_aufgabe_2_unit_tests

Enthalten ist jeweils eine simple Anwendung mit Unit-Tests in **Java**
([Maven], [JUnit]) und **Python** ([unittest]), bestehend aus einer `Town`-Klasse
mit zugehörigen Unit-Tests. Diese sind dem Praktikum zu "Unit-Test mit Java und
Python" entnommen.

[Maven]: https://maven.apache.org/
[JUnit]: https://junit.org/junit5/docs/current/user-guide/
[unittest]: https://docs.python.org/3/library/unittest.html

> [!WARNING]
> Ein Test schlägt bewusst fehl, da dieser im Zuge der Aufgabenbearbeitung
> angepasst wird.

## Hinweise

Die Beispielprojekte für **Java** und **Python** liegen in jeweils getrennten
Verzeichnissen:

```txt
├── example-java
└── example-python
```

Für jedes Projekt werden im Laufe der Bearbeitung der Aufgaben die zugehörigen
**GitHub Action Workflows** im Verzeichnis `.github/workflows/` angelegt.

## Example Java

### Projektstruktur (Java)

```txt
example-java
├── pom.xml
└── src
    ├── main
    │   └── java
    │       └── example
    │           ├── Main.java
    │           └── Town.java
    └── test
        └── java
            └── example
                ├── TestMain.java
                └── TestTown.java
```

| Datei           | Beschreibung                                                                                              |
| --------------- | --------------------------------------------------------------------------------------------------------- |
| `pom.xml`       | Beschreibung des Maven-Projektes im XML-Format                                                            |
| `Main.java`     | Klasse `Main` für die `main()` Methode                                                                    |
| `Town.java`     | Klasse `Town` für eine Stadt mit Name (`name`) und Einwohneranzahl (`residents`) und zugehörigen Methoden |
| `TestMain.java` | Klasse `TestMain` mit beispielhaftem Unit-Test zum Testen der `Main`-Klasse                               |
| `TestTown.java` | Klasse `TestTown` mit beispielhaften Unit-Tests zum Testen der `Town`-Klasse                              |

### Hinweise zu Java mit Maven

Für eine lokale Ausführung der Tests mit Maven wird eine [Maven-Installation][Maven]
benötigt (für das Praktikum nicht notwendig).

Im Folgenden ein paar hilfreiche Kommandos (vorher ggf. `cd example-java`):

- Version anzeigen:

  ```sh
  mvn --version
  ```

- Bereinigt die von Maven während des Builds generierten Dateien und
  Verzeichnisse (bspw. das _target_-Verzeichnis):

  ```sh
  mvn clean
  ```

- Führt Tests mit Maven aus:

  ```sh
  # Run all tests
  mvn test

  # Remove previous build artifacts and run all tests
  mvn clean test
  ```

- Führt gezielt ausgewählte Tests (Klassen, Methoden) aus:

  ```sh
  # Run a specific test class
  mvn -Dtest=TestTown test

  # Run multiple test classes
  mvn -Dtest=TestTown,TestCity test

  # Run a specific test method
  mvn -Dtest=TestTown#testTownValues test

  # Run tests matching a pattern
  mvn -Dtest=*TownTest test
  ```

- Kompiliert und erstellt ein Paket (bspw. JAR-Datei) aus den Quellcodedateien:

  ```sh
  mvn package
  ```

- Ausführen der erstellten JAR-Datei:

  ```console
  % java -cp target/example-1.0-SNAPSHOT.jar example.Main
  The town Dresden has 556780 residents.
  Town('Dresden', 556780)
  Dresden
  -1
  ```

## Example Python

### Projektstruktur (Python)

```txt
example-python
├── main
│   ├── __init__.py
│   ├── main.py
│   └── town.py
└── tests
    ├── __init__.py
    └── test_town.py
```

| Datei          | Beschreibung                                                                                              |
| -------------- | --------------------------------------------------------------------------------------------------------- |
| `__init__.py`  | Markiert das Verzeichnis als Paket (regular package) mit zugehörigem Namensraum (namespace)               |
| `main.py`      | Funktion `main()` zum Ausführen der Anwendung                                                             |
| `town.py`      | Klasse `Town` für eine Stadt mit Name (`name`) und Einwohneranzahl (`residents`) und zugehörigen Methoden |
| `test_town.py` | Klasse `TestTown` mit beispielhaften Unit-Tests zum Testen der `Town`-Klasse                              |

### Hinweise zu Python mit unittest

Im Folgenden ein paar hilfreiche Kommandos (vorher ggf. `cd example-python`):

- Version anzeigen:

  ```sh
  python3 --version
  ```

- Führt automatisch gefundene Tests aus:

  ```sh
  # Run tests using the default test discovery behavior
  python3 -m unittest

  # Discover tests recursively from the current directory
  python3 -m unittest discover

  # Discover tests in a specific directory
  python3 -m unittest discover -s tests

  # Discover tests in a specific directory with a specific filename pattern
  python3 -m unittest discover -s tests -p "test_*.py"
  ```

- Führt gezielt ausgewählte Tests (Module, Klassen, Methoden) aus:

  ```sh
  # Run all tests in a test module
  python3 -m unittest tests.test_town

  # Run all test methods in a specific test class
  python3 -m unittest tests.test_town.TestTown

  # Run one specific test method
  python3 -m unittest tests.test_town.TestTown.test_town_values
  ```

- Ausführen mit:

  ```console
  % python3 main/main.py
  The town Dresden has 556780 residents.
  Town('Dresden', 556780)
  Dresden
  -1
  ```
