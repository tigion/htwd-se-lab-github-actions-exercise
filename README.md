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

  ```shell
  % mvn --version
  ```

- Bereinigt die von Maven während des Builds generierten Dateien und
  Verzeichnisse (bspw. das _target_-Verzeichnis):

  ```shell
  % mvn clean
  ```

- Führt alle oder ausgewählte Test-Klassen aus:

  ```shell
  % mvn test
  % mvn clean test
  % mvn -Dtest=TestTown test
  ```

- Kompiliert und erstellt ein Paket (bspw. JAR-Datei) aus den Quellcodedateien:

  ```shell
  % mvn package
  ```

- Ausführen der erstellten JAR-Datei:

  ```shell
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

  ```shell
  % python3 --version
  ```

- Führt ausgewählte Tests, Test-Klassen/-Methoden aus:

  ```shell
  % python3 -m unittest tests.test_town
  % python3 -m unittest tests.test_town.TestTown
  % python3 -m unittest tests.test_town.TestTown.test_town_values
  ```

- Führt Tests aller oder ausgewählter Pakete/Dateien aus:

  ```shell
  % python3 -m unittest discover
  % python3 -m unittest discover tests
  % python3 -m unittest discover tests "test_*.py"
  ```

- Ausführen mit:

  ```shell
  % python3 main/main.py
  The town Dresden has 556780 residents.
  Town('Dresden', 556780)
  Dresden
  -1
  ```
