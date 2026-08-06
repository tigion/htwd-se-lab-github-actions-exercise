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
Verzeichnissen mit erklärenden `README.md`-Dateien:

```txt
├── example-java
└── example-python
```

Für jedes Projekt werden im Laufe der Bearbeitung der Aufgaben die zugehörigen
**GitHub Action Workflows** im Verzeichnis `.github/workflows/` angelegt.
